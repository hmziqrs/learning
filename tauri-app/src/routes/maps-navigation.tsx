import { createFileRoute } from '@tanstack/react-router'
import { Map, Navigation, Search, Route as RouteIcon, MapPin, Compass, Layers } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { useState, useEffect, useRef } from 'react'

export const Route = createFileRoute('/maps-navigation')({
  component: MapsNavigationModule,
})

interface LatLng {
  lat: number
  lng: number
}

interface RouteData {
  summary: {
    distance: string
    duration: string
  }
  instructions: string[]
}

interface SearchResult {
  id: string
  name: string
  address: string
  location: LatLng
}

function MapsNavigationModule() {
  const [output, setOutput] = useState<string[]>([])
  const [searchQuery, setSearchQuery] = useState('')
  const [searchResults, setSearchResults] = useState<SearchResult[]>([])
  const [currentLocation, setCurrentLocation] = useState<LatLng | null>(null)
  const [destination, setDestination] = useState<LatLng | null>(null)
  const [route, setRoute] = useState<RouteData | null>(null)
  const [isLoadingRoute, setIsLoadingRoute] = useState(false)
  const [mapInitialized, setMapInitialized] = useState(false)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Get current location
  const handleGetCurrentLocation = () => {
    if (!('geolocation' in navigator)) {
      addOutput('Geolocation is not supported by this browser', false)
      return
    }

    addOutput('Requesting current location...')

    navigator.geolocation.getCurrentPosition(
      (position) => {
        const location: LatLng = {
          lat: position.coords.latitude,
          lng: position.coords.longitude,
        }

        setCurrentLocation(location)
        addOutput(`Current location: ${location.lat.toFixed(6)}, ${location.lng.toFixed(6)}`)
      },
      (error) => {
        let errorMsg = 'Unknown error'
        switch (error.code) {
          case error.PERMISSION_DENIED:
            errorMsg = 'Location permission denied'
            break
          case error.POSITION_UNAVAILABLE:
            errorMsg = 'Location unavailable'
            break
          case error.TIMEOUT:
            errorMsg = 'Location request timeout'
            break
        }
        addOutput(`Failed to get location: ${errorMsg}`, false)
      },
      {
        enableHighAccuracy: true,
        timeout: 10000,
        maximumAge: 0,
      }
    )
  }

  // Search for address (simulated)
  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      addOutput('Please enter a search query', false)
      return
    }

    addOutput(`Searching for: ${searchQuery}`)

    try {
      // Using Nominatim for geocoding
      const url = `https://nominatim.openstreetmap.org/search?q=${encodeURIComponent(searchQuery)}&format=json&limit=5`

      const response = await fetch(url, {
        headers: {
          'User-Agent': 'TauriCapabilityPlayground/1.0',
        },
      })

      const data = await response.json()

      if (data.length === 0) {
        addOutput('No results found', false)
        setSearchResults([])
        return
      }

      const results: SearchResult[] = data.map((item: any, index: number) => ({
        id: `result-${index}`,
        name: item.display_name.split(',')[0],
        address: item.display_name,
        location: {
          lat: parseFloat(item.lat),
          lng: parseFloat(item.lon),
        },
      }))

      setSearchResults(results)
      addOutput(`Found ${results.length} results`)
    } catch (error) {
      addOutput(`Search failed: ${error}`, false)
      setSearchResults([])
    }
  }

  // Set destination from search result
  const handleSelectDestination = (result: SearchResult) => {
    setDestination(result.location)
    addOutput(`Destination set: ${result.name}`)
    setSearchResults([])
    setSearchQuery('')
  }

  // Calculate route (simulated)
  const handleCalculateRoute = async () => {
    if (!currentLocation) {
      addOutput('Please get your current location first', false)
      return
    }

    if (!destination) {
      addOutput('Please set a destination', false)
      return
    }

    setIsLoadingRoute(true)
    addOutput('Calculating route...')

    try {
      // Using OSRM for routing
      const coords = `${currentLocation.lng},${currentLocation.lat};${destination.lng},${destination.lat}`
      const url = `https://router.project-osrm.org/route/v1/driving/${coords}?overview=full&steps=true`

      const response = await fetch(url)
      const data = await response.json()

      if (data.code !== 'Ok') {
        throw new Error('Route calculation failed')
      }

      const routeData = data.routes[0]
      const distance = (routeData.distance / 1000).toFixed(1)
      const duration = Math.round(routeData.duration / 60)

      const instructions = routeData.legs[0].steps.map((step: any, index: number) => {
        const instruction = step.maneuver?.instruction || step.name || 'Continue'
        return `${index + 1}. ${instruction}`
      })

      const calculatedRoute: RouteData = {
        summary: {
          distance: `${distance} km`,
          duration: `${duration} min`,
        },
        instructions: instructions,
      }

      setRoute(calculatedRoute)
      addOutput(`Route calculated: ${distance} km, ${duration} min`)
    } catch (error) {
      addOutput(`Route calculation failed: ${error}`, false)
    } finally {
      setIsLoadingRoute(false)
    }
  }

  // Clear route
  const handleClearRoute = () => {
    setRoute(null)
    setDestination(null)
    setCurrentLocation(null)
    addOutput('Route cleared')
  }

  // Initialize map placeholder
  const handleInitializeMap = () => {
    setMapInitialized(true)
    addOutput('Map initialized (Leaflet integration required)')
  }

  return (
    <ModulePageLayout
      title="Maps & Navigation Module"
      description="Interactive maps, routing, turn-by-turn navigation, and geocoding services"
      icon={Map}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-blue-500">ℹ️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-green-600">✓ Geolocation API</strong> - Web API for current location
              </li>
              <li>
                <strong className="text-green-600">✓ Geocoding</strong> - Nominatim (OpenStreetMap) address search
              </li>
              <li>
                <strong className="text-green-600">✓ Routing</strong> - OSRM for route calculation
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Map Display</strong> - Requires Leaflet.js integration
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Turn-by-Turn Navigation</strong> - Requires location tracking
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Install Leaflet for interactive maps:</div>
              <div>bun add leaflet @types/leaflet -D</div>
              <div>bun add leaflet-routing-machine</div>
            </div>
            <p className="text-muted-foreground mt-2">
              This demo uses free services: Nominatim for geocoding and OSRM for routing.
            </p>
          </div>
        </section>

        {/* Map Display Placeholder */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <Map className="w-5 h-5" />
              Interactive Map
            </h2>
            {!mapInitialized && (
              <Button onClick={handleInitializeMap} variant="outline" size="sm">
                Initialize Map
              </Button>
            )}
          </div>

          <div className="bg-muted/30 rounded-md border-2 border-dashed min-h-[400px] flex flex-col items-center justify-center p-8 text-center">
            {!mapInitialized ? (
              <>
                <Map className="w-16 h-16 text-muted-foreground mb-4" />
                <p className="text-muted-foreground mb-2">
                  Map integration requires Leaflet.js
                </p>
                <p className="text-xs text-muted-foreground">
                  Install: <code className="bg-muted px-2 py-1 rounded">bun add leaflet</code>
                </p>
              </>
            ) : (
              <>
                <Layers className="w-16 h-16 text-muted-foreground mb-4" />
                <p className="text-muted-foreground mb-2">Map Preview</p>
                {currentLocation && (
                  <div className="space-y-1 text-sm">
                    <p className="flex items-center gap-2 text-green-600">
                      <MapPin className="w-4 h-4" />
                      Current Location: {currentLocation.lat.toFixed(4)}, {currentLocation.lng.toFixed(4)}
                    </p>
                  </div>
                )}
                {destination && (
                  <p className="flex items-center gap-2 text-blue-600 text-sm mt-2">
                    <Navigation className="w-4 h-4" />
                    Destination: {destination.lat.toFixed(4)}, {destination.lng.toFixed(4)}
                  </p>
                )}
                {route && (
                  <div className="mt-4 p-3 bg-blue-500/10 border border-blue-500/30 rounded-md">
                    <p className="text-sm font-semibold text-blue-600">Route would be displayed here</p>
                    <p className="text-xs text-muted-foreground mt-1">
                      Blue polyline showing the route path
                    </p>
                  </div>
                )}
              </>
            )}
          </div>
        </section>

        {/* Location Controls */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <MapPin className="w-5 h-5" />
            Current Location
          </h2>

          <div className="flex flex-wrap gap-2">
            <Button onClick={handleGetCurrentLocation} variant="outline">
              <Compass className="w-4 h-4 mr-2" />
              Get Current Location
            </Button>
          </div>

          {currentLocation && (
            <div className="bg-muted rounded-md p-4 space-y-2">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <span className="text-sm text-muted-foreground">Latitude:</span>
                  <p className="font-mono font-semibold">{currentLocation.lat.toFixed(6)}</p>
                </div>
                <div>
                  <span className="text-sm text-muted-foreground">Longitude:</span>
                  <p className="font-mono font-semibold">{currentLocation.lng.toFixed(6)}</p>
                </div>
              </div>
            </div>
          )}
        </section>

        {/* Address Search */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Search className="w-5 h-5" />
            Address Search & Geocoding
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Search for addresses, places, and points of interest
            </p>

            <div className="flex gap-2">
              <Input
                type="text"
                placeholder="Search for address or place..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                    handleSearch()
                  }
                }}
                className="flex-1"
              />
              <Button onClick={handleSearch} variant="default">
                <Search className="w-4 h-4 mr-2" />
                Search
              </Button>
            </div>

            {searchResults.length > 0 && (
              <div className="bg-muted rounded-md divide-y divide-border max-h-64 overflow-y-auto">
                {searchResults.map((result) => (
                  <button
                    key={result.id}
                    onClick={() => handleSelectDestination(result)}
                    className="w-full p-3 text-left hover:bg-muted-foreground/10 transition-colors"
                  >
                    <div className="font-semibold text-sm">{result.name}</div>
                    <div className="text-xs text-muted-foreground mt-1">{result.address}</div>
                  </button>
                ))}
              </div>
            )}
          </div>
        </section>

        {/* Route Calculation */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <RouteIcon className="w-5 h-5" />
            Route Calculation
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Calculate routes between your current location and destination
            </p>

            <div className="flex flex-wrap gap-2">
              <Button
                onClick={handleCalculateRoute}
                disabled={!currentLocation || !destination || isLoadingRoute}
                variant="default"
              >
                <Navigation className="w-4 h-4 mr-2" />
                {isLoadingRoute ? 'Calculating...' : 'Calculate Route'}
              </Button>

              <Button onClick={handleClearRoute} variant="outline">
                Clear Route
              </Button>
            </div>

            {!currentLocation && (
              <p className="text-sm text-yellow-600 dark:text-yellow-400">
                ⚠ Get your current location first
              </p>
            )}

            {!destination && currentLocation && (
              <p className="text-sm text-yellow-600 dark:text-yellow-400">
                ⚠ Search and select a destination
              </p>
            )}
          </div>

          {route && (
            <div className="mt-4 space-y-4">
              {/* Route Summary */}
              <div className="bg-blue-500/10 border border-blue-500/30 rounded-md p-4">
                <h3 className="font-semibold mb-3 flex items-center gap-2 text-blue-600">
                  <Navigation className="w-4 h-4" />
                  Route Summary
                </h3>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <span className="text-sm text-muted-foreground">Distance:</span>
                    <p className="font-semibold text-lg">{route.summary.distance}</p>
                  </div>
                  <div>
                    <span className="text-sm text-muted-foreground">Duration:</span>
                    <p className="font-semibold text-lg">{route.summary.duration}</p>
                  </div>
                </div>
              </div>

              {/* Turn-by-Turn Instructions */}
              <div className="rounded-md border">
                <div className="bg-muted px-4 py-3 font-semibold">
                  Turn-by-Turn Directions
                </div>
                <div className="divide-y divide-border max-h-96 overflow-y-auto">
                  {route.instructions.map((instruction, index) => (
                    <div key={index} className="p-3 hover:bg-muted/50 transition-colors">
                      <p className="text-sm">{instruction}</p>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
        </section>

        {/* Features Overview */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold">Available Features</h2>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="border rounded-lg p-4 space-y-2">
              <div className="flex items-center gap-2 text-green-600">
                <div className="w-2 h-2 bg-green-600 rounded-full" />
                <h3 className="font-semibold">Implemented</h3>
              </div>
              <ul className="space-y-1 text-sm text-muted-foreground ml-4 list-disc">
                <li>Current location via Geolocation API</li>
                <li>Address search and geocoding</li>
                <li>Route calculation with OSRM</li>
                <li>Turn-by-turn directions</li>
                <li>Distance and duration estimates</li>
              </ul>
            </div>

            <div className="border rounded-lg p-4 space-y-2">
              <div className="flex items-center gap-2 text-yellow-600">
                <div className="w-2 h-2 bg-yellow-600 rounded-full" />
                <h3 className="font-semibold">Planned</h3>
              </div>
              <ul className="space-y-1 text-sm text-muted-foreground ml-4 list-disc">
                <li>Interactive map with Leaflet.js</li>
                <li>Visual route display on map</li>
                <li>Custom markers and icons</li>
                <li>Geofencing support</li>
                <li>Offline map tiles</li>
                <li>Real-time navigation</li>
                <li>Traffic overlay</li>
                <li>Points of Interest search</li>
              </ul>
            </div>
          </div>
        </section>

        {/* Output Panel */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold">Output</h2>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>

          <div className="bg-muted rounded-md p-4 h-64 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No output yet...</p>
            ) : (
              output.map((line, i) => (
                <div key={i} className="mb-1">
                  {line}
                </div>
              ))
            )}
          </div>
        </section>

        {/* Implementation Guide */}
        <section className="rounded-lg border border-purple-500/50 bg-purple-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Implementation Guide</h3>
          <div className="space-y-4 text-sm">
            <div className="space-y-2">
              <h4 className="font-semibold">Map Integration (Leaflet.js)</h4>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>import L from 'leaflet'</div>
                <div>import 'leaflet/dist/leaflet.css'</div>
                <div className="mt-2">const map = L.map('map').setView([lat, lng], 13)</div>
                <div>L.tileLayer('https://&#123;s&#125;.tile.openstreetmap.org/&#123;z&#125;/&#123;x&#125;/&#123;y&#125;.png').addTo(map)</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Routing Services</h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
                <li><strong>OSRM</strong> - Free, open source, used in this demo</li>
                <li><strong>OpenRouteService</strong> - Free tier with API key</li>
                <li><strong>Mapbox Directions</strong> - Premium, 100k requests/month free</li>
                <li><strong>Google Maps</strong> - Premium, pay-per-use</li>
              </ul>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Next Steps</h4>
              <ul className="list-decimal list-inside space-y-1 text-muted-foreground ml-2">
                <li>Install Leaflet.js for map display</li>
                <li>Integrate map component into this page</li>
                <li>Add visual route rendering</li>
                <li>Implement real-time location tracking</li>
                <li>Add geofencing capabilities</li>
                <li>Implement offline map support</li>
              </ul>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
