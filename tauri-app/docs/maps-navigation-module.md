# Maps & Navigation Module

## Overview

The Maps & Navigation Module provides comprehensive mapping, routing, and navigation capabilities for the application. Display interactive maps, calculate routes, provide turn-by-turn navigation, search for points of interest, and implement geofencing features across desktop and mobile platforms.

## Current Implementation Status

⚠️ **Planned** - Requires map library integration and routing services

## Plugin Setup

### Map Libraries

Several options available for map integration:

**Leaflet.js (Recommended for simplicity):**
- Lightweight and mobile-friendly
- Free and open source
- Works with OpenStreetMap tiles
- Extensive plugin ecosystem

**Mapbox GL JS:**
- Modern vector-based maps
- Rich interactivity and styling
- Free tier available
- Excellent mobile performance

**Google Maps Platform:**
- Comprehensive features
- Familiar interface
- Premium pricing
- Extensive documentation

**HERE Maps:**
- Strong offline capabilities
- Good routing engine
- Enterprise-focused
- Free tier available

### Dependencies

```bash
# Leaflet.js (recommended starting point)
bun add leaflet @types/leaflet -D

# Leaflet routing plugin
bun add leaflet-routing-machine

# For geocoding
bun add nominatim-browser

# Alternative: Mapbox GL JS
bun add mapbox-gl @types/mapbox-gl -D

# For offline tile caching
bun add leaflet.offline
```

### Routing Services

**OpenRouteService (Free):**
- Open source routing engine
- Multiple routing profiles
- Turn-by-turn instructions
- Free API tier

**OSRM (Free, self-hosted):**
- Fast routing based on OpenStreetMap
- Can be self-hosted
- No API key required

**Mapbox Directions API:**
- High-quality routing
- Real-time traffic
- Free tier: 100,000 requests/month

**Google Maps Directions API:**
- Accurate routing
- Real-time traffic
- Paid service

### Geocoding Services

**Nominatim (Free):**
- OpenStreetMap geocoding
- Free to use with attribution
- Rate-limited

**Mapbox Geocoding API:**
- Fast and accurate
- Free tier available

**Google Geocoding API:**
- High accuracy
- Paid service

## Permissions Configuration

### Android Manifest

```xml
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
<uses-permission android:name="android.permission.INTERNET" />
```

### iOS Info.plist

```xml
<key>NSLocationWhenInUseUsageDescription</key>
<string>We need your location to provide navigation and show you on the map</string>
<key>NSLocationAlwaysUsageDescription</key>
<string>We need your location for turn-by-turn navigation</string>
```

### Tauri Capabilities

```json
{
  "permissions": [
    "core:default",
    "geolocation:allow-current-position",
    "geolocation:allow-watch-position"
  ]
}
```

## Core Features

### Map Display & Interaction
- [ ] Display interactive map with pan and zoom
- [ ] Multiple map tile providers (OpenStreetMap, Satellite, etc.)
- [ ] Custom map styling and themes
- [ ] Marker placement and management
- [ ] Polyline and polygon drawing
- [ ] Map clustering for multiple markers
- [ ] Geolocation marker (current position)
- [ ] Compass and zoom controls
- [ ] Scale indicator
- [ ] Attribution display

### Routing & Navigation
- [ ] Calculate routes between two points
- [ ] Multi-waypoint routing
- [ ] Alternative route suggestions
- [ ] Route optimization
- [ ] Distance and duration estimation
- [ ] Turn-by-turn directions
- [ ] Voice navigation guidance
- [ ] Real-time route tracking
- [ ] Route deviation detection
- [ ] ETA calculation and updates

### Search & Geocoding
- [ ] Address search and autocomplete
- [ ] Reverse geocoding (coordinates to address)
- [ ] Forward geocoding (address to coordinates)
- [ ] Points of Interest (POI) search
- [ ] Category-based search
- [ ] Search within radius
- [ ] Search along route

### Geofencing
- [ ] Create circular geofences
- [ ] Create polygon geofences
- [ ] Geofence entry notifications
- [ ] Geofence exit notifications
- [ ] Multiple geofence monitoring
- [ ] Geofence visualization on map

### Offline Capabilities
- [ ] Download map tiles for offline use
- [ ] Cache routes and directions
- [ ] Offline geocoding database
- [ ] Storage management for offline data
- [ ] Update offline map data

### Traffic & Real-time Data
- [ ] Real-time traffic overlay
- [ ] Traffic incident markers
- [ ] Route recalculation based on traffic
- [ ] Live traffic updates

### Customization
- [ ] Custom map markers and icons
- [ ] Custom route styling
- [ ] Info windows and popups
- [ ] Drawing tools (measure distance, area)
- [ ] Layer management and switching

## Data Structures

### Map Configuration

```typescript
interface MapConfig {
  center: LatLng
  zoom: number
  minZoom?: number
  maxZoom?: number
  tileLayer: TileLayerConfig
  enableGeolocation?: boolean
  enableClustering?: boolean
}

interface LatLng {
  lat: number
  lng: number
}

interface TileLayerConfig {
  url: string
  attribution: string
  maxZoom: number
  subdomains?: string[]
}
```

### Route Data

```typescript
interface Route {
  id: string
  summary: RouteSummary
  coordinates: LatLng[]
  instructions: RouteInstruction[]
  waypoints: Waypoint[]
  geometry: string // encoded polyline
  alternatives?: Route[]
}

interface RouteSummary {
  distance: number // meters
  duration: number // seconds
  distanceText: string // "5.2 km"
  durationText: string // "12 min"
}

interface RouteInstruction {
  type: InstructionType
  distance: number
  duration: number
  text: string
  streetName: string
  direction?: string // "left", "right", "straight"
  coordinate: LatLng
}

type InstructionType =
  | 'turn-left'
  | 'turn-right'
  | 'turn-slight-left'
  | 'turn-slight-right'
  | 'turn-sharp-left'
  | 'turn-sharp-right'
  | 'continue-straight'
  | 'enter-roundabout'
  | 'exit-roundabout'
  | 'u-turn'
  | 'arrive'
  | 'depart'

interface Waypoint {
  location: LatLng
  name?: string
  type: 'start' | 'waypoint' | 'destination'
}
```

### Geocoding Results

```typescript
interface GeocodingResult {
  address: string
  location: LatLng
  type: string // "address", "poi", "city", etc.
  importance: number
  boundingBox?: BoundingBox
  displayName: string
  components: AddressComponents
}

interface AddressComponents {
  houseNumber?: string
  street?: string
  city?: string
  state?: string
  country?: string
  postalCode?: string
}

interface BoundingBox {
  southwest: LatLng
  northeast: LatLng
}
```

### Point of Interest

```typescript
interface PointOfInterest {
  id: string
  name: string
  location: LatLng
  category: POICategory
  address: string
  rating?: number
  phone?: string
  website?: string
  openingHours?: string
  description?: string
}

type POICategory =
  | 'restaurant'
  | 'gas-station'
  | 'parking'
  | 'hotel'
  | 'hospital'
  | 'pharmacy'
  | 'atm'
  | 'shopping'
  | 'entertainment'
  | 'other'
```

### Geofence

```typescript
interface Geofence {
  id: string
  name: string
  shape: GeofenceShape
  notifyOnEntry: boolean
  notifyOnExit: boolean
  active: boolean
}

type GeofenceShape = CircularGeofence | PolygonGeofence

interface CircularGeofence {
  type: 'circle'
  center: LatLng
  radius: number // meters
}

interface PolygonGeofence {
  type: 'polygon'
  coordinates: LatLng[]
}

interface GeofenceEvent {
  geofenceId: string
  geofenceName: string
  type: 'entry' | 'exit'
  location: LatLng
  timestamp: number
}
```

### Map Marker

```typescript
interface MapMarker {
  id: string
  location: LatLng
  title?: string
  description?: string
  icon?: MarkerIcon
  draggable?: boolean
  onClick?: () => void
}

interface MarkerIcon {
  url: string
  size: { width: number; height: number }
  anchor?: { x: number; y: number }
}
```

## Frontend Implementation

### Leaflet Integration Example

```typescript
import L from 'leaflet'
import 'leaflet/dist/leaflet.css'
import { useEffect, useRef, useState } from 'react'

interface MapComponentProps {
  center?: LatLng
  zoom?: number
  markers?: MapMarker[]
  route?: Route
}

export function MapComponent({
  center = { lat: 40.7128, lng: -74.0060 },
  zoom = 13,
  markers = [],
  route
}: MapComponentProps) {
  const mapRef = useRef<L.Map | null>(null)
  const mapContainerRef = useRef<HTMLDivElement>(null)
  const markersRef = useRef<L.Marker[]>([])
  const routeLayerRef = useRef<L.Polyline | null>(null)

  // Initialize map
  useEffect(() => {
    if (!mapContainerRef.current || mapRef.current) return

    const map = L.map(mapContainerRef.current).setView([center.lat, center.lng], zoom)

    // Add tile layer
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '© <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      maxZoom: 19,
    }).addTo(map)

    mapRef.current = map

    return () => {
      map.remove()
      mapRef.current = null
    }
  }, [])

  // Update markers
  useEffect(() => {
    if (!mapRef.current) return

    // Clear existing markers
    markersRef.current.forEach(marker => marker.remove())
    markersRef.current = []

    // Add new markers
    markers.forEach(markerData => {
      const marker = L.marker([markerData.location.lat, markerData.location.lng])
        .addTo(mapRef.current!)

      if (markerData.title || markerData.description) {
        const popup = L.popup()
          .setContent(`
            ${markerData.title ? `<h3>${markerData.title}</h3>` : ''}
            ${markerData.description ? `<p>${markerData.description}</p>` : ''}
          `)
        marker.bindPopup(popup)
      }

      if (markerData.onClick) {
        marker.on('click', markerData.onClick)
      }

      markersRef.current.push(marker)
    })
  }, [markers])

  // Update route
  useEffect(() => {
    if (!mapRef.current) return

    // Remove existing route
    if (routeLayerRef.current) {
      routeLayerRef.current.remove()
      routeLayerRef.current = null
    }

    // Add new route
    if (route) {
      const latLngs = route.coordinates.map(coord => [coord.lat, coord.lng] as [number, number])
      const polyline = L.polyline(latLngs, {
        color: '#2563eb',
        weight: 5,
        opacity: 0.7,
      }).addTo(mapRef.current)

      // Fit map to route bounds
      mapRef.current.fitBounds(polyline.getBounds(), {
        padding: [50, 50],
      })

      routeLayerRef.current = polyline
    }
  }, [route])

  return <div ref={mapContainerRef} className="w-full h-full" />
}
```

### Routing Hook

```typescript
import { useState } from 'react'

interface UseRoutingOptions {
  apiKey?: string
  service?: 'openrouteservice' | 'osrm' | 'mapbox'
}

export function useRouting(options: UseRoutingOptions = {}) {
  const [route, setRoute] = useState<Route | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const calculateRoute = async (
    start: LatLng,
    end: LatLng,
    waypoints: LatLng[] = []
  ) => {
    setLoading(true)
    setError(null)

    try {
      // Example using OSRM (free, no API key)
      const coords = [start, ...waypoints, end]
        .map(point => `${point.lng},${point.lat}`)
        .join(';')

      const url = `https://router.project-osrm.org/route/v1/driving/${coords}?overview=full&steps=true`

      const response = await fetch(url)
      const data = await response.json()

      if (data.code !== 'Ok') {
        throw new Error('Failed to calculate route')
      }

      const routeData = data.routes[0]

      // Parse route data
      const parsedRoute: Route = {
        id: crypto.randomUUID(),
        summary: {
          distance: routeData.distance,
          duration: routeData.duration,
          distanceText: `${(routeData.distance / 1000).toFixed(1)} km`,
          durationText: `${Math.round(routeData.duration / 60)} min`,
        },
        coordinates: decodePolyline(routeData.geometry),
        instructions: routeData.legs[0].steps.map((step: any) => ({
          type: mapInstructionType(step.maneuver.type),
          distance: step.distance,
          duration: step.duration,
          text: step.maneuver.instruction || step.name,
          streetName: step.name,
          direction: step.maneuver.modifier,
          coordinate: { lat: step.maneuver.location[1], lng: step.maneuver.location[0] },
        })),
        waypoints: coords.map((_, index) => ({
          location: index === 0 ? start : index === coords.length - 1 ? end : waypoints[index - 1],
          type: index === 0 ? 'start' : index === coords.length - 1 ? 'destination' : 'waypoint',
        })),
        geometry: routeData.geometry,
      }

      setRoute(parsedRoute)
      return parsedRoute
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Unknown error'
      setError(message)
      throw err
    } finally {
      setLoading(false)
    }
  }

  const clearRoute = () => {
    setRoute(null)
    setError(null)
  }

  return {
    route,
    loading,
    error,
    calculateRoute,
    clearRoute,
  }
}

// Helper function to decode polyline
function decodePolyline(encoded: string): LatLng[] {
  const coordinates: LatLng[] = []
  let index = 0
  let lat = 0
  let lng = 0

  while (index < encoded.length) {
    let shift = 0
    let result = 0
    let byte: number

    do {
      byte = encoded.charCodeAt(index++) - 63
      result |= (byte & 0x1f) << shift
      shift += 5
    } while (byte >= 0x20)

    const deltaLat = result & 1 ? ~(result >> 1) : result >> 1
    lat += deltaLat

    shift = 0
    result = 0

    do {
      byte = encoded.charCodeAt(index++) - 63
      result |= (byte & 0x1f) << shift
      shift += 5
    } while (byte >= 0x20)

    const deltaLng = result & 1 ? ~(result >> 1) : result >> 1
    lng += deltaLng

    coordinates.push({
      lat: lat / 1e5,
      lng: lng / 1e5,
    })
  }

  return coordinates
}

function mapInstructionType(osrmType: string): InstructionType {
  const mapping: Record<string, InstructionType> = {
    'turn': 'turn-left',
    'new name': 'continue-straight',
    'depart': 'depart',
    'arrive': 'arrive',
    'roundabout': 'enter-roundabout',
    'rotary': 'enter-roundabout',
  }
  return mapping[osrmType] || 'continue-straight'
}
```

### Geocoding Hook

```typescript
export function useGeocoding() {
  const [results, setResults] = useState<GeocodingResult[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const searchAddress = async (query: string) => {
    setLoading(true)
    setError(null)

    try {
      // Using Nominatim (OpenStreetMap geocoding service)
      const url = `https://nominatim.openstreetmap.org/search?q=${encodeURIComponent(query)}&format=json&limit=5`

      const response = await fetch(url, {
        headers: {
          'User-Agent': 'TauriCapabilityPlayground/1.0',
        },
      })

      const data = await response.json()

      const geocodingResults: GeocodingResult[] = data.map((item: any) => ({
        address: item.display_name,
        location: { lat: parseFloat(item.lat), lng: parseFloat(item.lon) },
        type: item.type,
        importance: item.importance,
        displayName: item.display_name,
        boundingBox: item.boundingbox ? {
          southwest: { lat: parseFloat(item.boundingbox[0]), lng: parseFloat(item.boundingbox[2]) },
          northeast: { lat: parseFloat(item.boundingbox[1]), lng: parseFloat(item.boundingbox[3]) },
        } : undefined,
        components: {
          street: item.address?.road,
          city: item.address?.city || item.address?.town,
          state: item.address?.state,
          country: item.address?.country,
          postalCode: item.address?.postcode,
        },
      }))

      setResults(geocodingResults)
      return geocodingResults
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Geocoding failed'
      setError(message)
      throw err
    } finally {
      setLoading(false)
    }
  }

  const reverseGeocode = async (location: LatLng) => {
    setLoading(true)
    setError(null)

    try {
      const url = `https://nominatim.openstreetmap.org/reverse?lat=${location.lat}&lon=${location.lng}&format=json`

      const response = await fetch(url, {
        headers: {
          'User-Agent': 'TauriCapabilityPlayground/1.0',
        },
      })

      const data = await response.json()

      const result: GeocodingResult = {
        address: data.display_name,
        location: { lat: parseFloat(data.lat), lng: parseFloat(data.lon) },
        type: data.type,
        importance: data.importance,
        displayName: data.display_name,
        components: {
          houseNumber: data.address?.house_number,
          street: data.address?.road,
          city: data.address?.city || data.address?.town,
          state: data.address?.state,
          country: data.address?.country,
          postalCode: data.address?.postcode,
        },
      }

      setResults([result])
      return result
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Reverse geocoding failed'
      setError(message)
      throw err
    } finally {
      setLoading(false)
    }
  }

  return {
    results,
    loading,
    error,
    searchAddress,
    reverseGeocode,
  }
}
```

## UI Components

### Navigation Panel
- [ ] Route summary card (distance, duration, ETA)
- [ ] Turn-by-turn instruction list
- [ ] Current instruction highlight
- [ ] Next instruction preview
- [ ] Reroute button
- [ ] Cancel navigation button

### Search Interface
- [ ] Address search input with autocomplete
- [ ] Search results list
- [ ] Recent searches history
- [ ] Favorite locations
- [ ] Category filters for POI

### Route Options
- [ ] Multiple route alternatives display
- [ ] Route preference selection (fastest, shortest, eco-friendly)
- [ ] Avoid toll roads toggle
- [ ] Avoid highways toggle
- [ ] Walking/driving/cycling mode selector

### Map Controls
- [ ] Zoom in/out buttons
- [ ] Compass/rotation control
- [ ] Current location button
- [ ] Layer switcher (map/satellite)
- [ ] Traffic overlay toggle
- [ ] Fullscreen toggle

## Testing Checklist

### Map Display
- [ ] Map loads correctly on all platforms
- [ ] Map tiles load and display properly
- [ ] Pan and zoom interactions work smoothly
- [ ] Custom markers display correctly
- [ ] Clustering works for multiple markers
- [ ] Map performance is acceptable with many markers

### Routing
- [ ] Routes calculate successfully
- [ ] Alternative routes are suggested
- [ ] Turn-by-turn instructions are accurate
- [ ] Route visualization on map is correct
- [ ] Waypoint routing works
- [ ] Route recalculation on deviation

### Geocoding
- [ ] Address search returns relevant results
- [ ] Autocomplete suggestions appear
- [ ] Reverse geocoding works accurately
- [ ] Search handles edge cases (special characters, etc.)

### Navigation
- [ ] Real-time location tracking during navigation
- [ ] Voice guidance works (if implemented)
- [ ] Route progress updates correctly
- [ ] ETA updates dynamically
- [ ] Navigation handles GPS signal loss

### Offline Features
- [ ] Offline map tiles can be downloaded
- [ ] Offline maps display correctly
- [ ] Cached routes work offline
- [ ] Storage limits are enforced

### Performance
- [ ] Map renders smoothly at 60fps
- [ ] Route calculation is fast (<2 seconds)
- [ ] Memory usage is acceptable
- [ ] Battery drain is reasonable during navigation

### Edge Cases
- [ ] Handle no internet connection gracefully
- [ ] Handle GPS unavailable
- [ ] Handle invalid addresses
- [ ] Handle route calculation failures
- [ ] Handle API rate limits

## Implementation Status

### Backend
- [ ] Location services integration
- [ ] Geofencing implementation
- [ ] Background location tracking
- [ ] Offline tile storage management
- [ ] Route caching system

### Frontend
- [ ] Map component with Leaflet
- [ ] Marker management system
- [ ] Route display and visualization
- [ ] Search interface with autocomplete
- [ ] Navigation UI components
- [ ] Turn-by-turn instruction display
- [ ] Geofence creation and management UI
- [ ] Settings for map preferences
- [ ] Offline map download UI

### Features
- [ ] Interactive map display
- [ ] Route calculation
- [ ] Turn-by-turn navigation
- [ ] Address search and geocoding
- [ ] Points of interest search
- [ ] Geofencing
- [ ] Offline maps
- [ ] Traffic overlay
- [ ] Multiple route alternatives
- [ ] Voice navigation

## Troubleshooting

### Map Not Displaying

**Issue**: Map container shows blank or tiles don't load

**Solutions**:
- Ensure Leaflet CSS is imported
- Set explicit height on map container element
- Check tile server URL is accessible
- Verify attribution is included
- Check browser console for CORS errors
- Ensure map is initialized after container is mounted

### Routes Not Calculating

**Issue**: Route calculation fails or returns errors

**Solutions**:
- Verify routing service API endpoint is accessible
- Check API key is valid (if required)
- Ensure coordinates are in correct format (lat, lng)
- Check network connectivity
- Verify routing service rate limits not exceeded
- Try alternative routing service

### Geolocation Not Working

**Issue**: Current location cannot be obtained

**Solutions**:
- Check location permissions are granted
- Verify app is served over HTTPS (required for geolocation API)
- Ensure location services are enabled on device
- Check GPS signal is available
- Handle permission denial gracefully
- Test on physical device (not emulator)

### Poor Map Performance

**Issue**: Map is slow or laggy

**Solutions**:
- Reduce number of markers (implement clustering)
- Use marker clustering for large datasets
- Optimize tile loading strategy
- Reduce map update frequency
- Use hardware acceleration
- Implement virtualization for marker lists
- Decrease zoom level transitions

### Offline Maps Not Working

**Issue**: Downloaded tiles don't display offline

**Solutions**:
- Verify tiles were downloaded successfully
- Check storage permissions
- Ensure tile cache is not corrupted
- Check storage space is sufficient
- Verify tile URL pattern is correct
- Test with a small area first

## Resources

### Map Libraries
- [Leaflet.js Documentation](https://leafletjs.com/)
- [Mapbox GL JS Documentation](https://docs.mapbox.com/mapbox-gl-js/)
- [Google Maps JavaScript API](https://developers.google.com/maps/documentation/javascript)
- [HERE Maps JavaScript API](https://developer.here.com/documentation/maps/3.1.41.0/dev_guide/index.html)

### Routing Services
- [OpenRouteService API](https://openrouteservice.org/)
- [OSRM API Documentation](http://project-osrm.org/)
- [Mapbox Directions API](https://docs.mapbox.com/api/navigation/directions/)
- [Google Directions API](https://developers.google.com/maps/documentation/directions)

### Geocoding Services
- [Nominatim (OpenStreetMap)](https://nominatim.org/)
- [Mapbox Geocoding API](https://docs.mapbox.com/api/search/geocoding/)
- [Google Geocoding API](https://developers.google.com/maps/documentation/geocoding)

### Tile Providers
- [OpenStreetMap](https://www.openstreetmap.org/)
- [Mapbox Studio](https://www.mapbox.com/mapbox-studio)
- [Thunderforest](https://www.thunderforest.com/)
- [Stamen Maps](http://maps.stamen.com/)

### Tools & Utilities
- [Leaflet Plugins](https://leafletjs.com/plugins.html)
- [Polyline Encoder/Decoder](https://developers.google.com/maps/documentation/utilities/polylinealgorithm)
- [GeoJSON.io](http://geojson.io/) - GeoJSON editor
- [Overpass Turbo](https://overpass-turbo.eu/) - OSM data extraction

## Platform Support

| Feature | Windows | macOS | Linux | iOS | Android |
|---------|---------|-------|-------|-----|---------|
| **Map Display** |
| Interactive Maps | ✅ | ✅ | ✅ | ✅ | ✅ |
| Custom Markers | ✅ | ✅ | ✅ | ✅ | ✅ |
| Tile Layers | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Routing** |
| Route Calculation | ✅ | ✅ | ✅ | ✅ | ✅ |
| Turn-by-Turn | ✅ | ✅ | ✅ | ✅ | ✅ |
| Voice Navigation | ✅* | ✅* | ✅* | ✅ | ✅ |
| **Search** |
| Address Search | ✅ | ✅ | ✅ | ✅ | ✅ |
| POI Search | ✅ | ✅ | ✅ | ✅ | ✅ |
| Autocomplete | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Geofencing** |
| Geofence Creation | ✅ | ✅ | ✅ | ✅ | ✅ |
| Background Monitoring | ❌ | ❌ | ❌ | ✅** | ✅** |
| **Offline** |
| Offline Maps | ✅ | ✅ | ✅ | ✅ | ✅ |
| Offline Routing | ✅*** | ✅*** | ✅*** | ✅*** | ✅*** |
| **Location** |
| Current Location | ✅ | ✅ | ✅ | ✅ | ✅ |
| Location Tracking | ✅ | ✅ | ✅ | ✅ | ✅ |

**Legend:**
- ✅ Fully Supported
- ✅* Requires Web Speech API or text-to-speech implementation
- ✅** Requires custom native plugin for background monitoring
- ✅*** Requires pre-downloaded routing data
- ❌ Not Supported

---

Last Updated: November 2025
Module Version: 1.0.0
Status: Planned
