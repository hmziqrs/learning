import { createFileRoute } from '@tanstack/react-router'
import { Activity, Compass, MapPin, Radio, Thermometer, Eye, Droplets, Gauge, Footprints } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect, useRef } from 'react'

export const Route = createFileRoute('/sensors')({
  component: SensorsModule,
})

interface SensorData {
  x: number
  y: number
  z: number
  timestamp: number
}

interface LocationData {
  latitude: number
  longitude: number
  accuracy: number
  altitude: number | null
  heading: number | null
  speed: number | null
  timestamp: number
}

interface ProximityData {
  isNear: boolean
  distance: number | null
  maxRange: number
  timestamp: number
}

interface LightData {
  illuminance: number
  timestamp: number
}

interface PressureData {
  pressure: number
  altitude: number | null
  timestamp: number
}

interface TemperatureData {
  celsius: number
  fahrenheit: number
  type: 'device' | 'ambient'
  timestamp: number
}

interface HumidityData {
  relativeHumidity: number
  timestamp: number
}

interface StepCounterData {
  steps: number
  timestamp: number
}

function SensorsModule() {
  const [output, setOutput] = useState<string[]>([])
  const [location, setLocation] = useState<LocationData | null>(null)
  const [isWatchingLocation, setIsWatchingLocation] = useState(false)
  const [accelerometerData, setAccelerometerData] = useState<SensorData | null>(null)
  const [proximityData, setProximityData] = useState<ProximityData | null>(null)
  const [lightData, setLightData] = useState<LightData | null>(null)
  const [pressureData, setPressureData] = useState<PressureData | null>(null)
  const [temperatureData, setTemperatureData] = useState<TemperatureData | null>(null)
  const [humidityData, setHumidityData] = useState<HumidityData | null>(null)
  const [stepCounterData, setStepCounterData] = useState<StepCounterData | null>(null)
  const [locationError, setLocationError] = useState<string | null>(null)
  const watchIdRef = useRef<number | null>(null)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const formatCoordinate = (value: number, decimals: number = 6): string => {
    return value.toFixed(decimals)
  }

  // Check if geolocation is supported
  const checkGeolocationSupport = (): boolean => {
    if (!('geolocation' in navigator)) {
      addOutput('Geolocation is not supported by this browser', false)
      return false
    }
    return true
  }

  // Get current position
  const handleGetCurrentLocation = () => {
    if (!checkGeolocationSupport()) return

    addOutput('Requesting current location...')
    setLocationError(null)

    navigator.geolocation.getCurrentPosition(
      (position) => {
        const locationData: LocationData = {
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
          accuracy: position.coords.accuracy,
          altitude: position.coords.altitude,
          heading: position.coords.heading,
          speed: position.coords.speed,
          timestamp: position.timestamp,
        }

        setLocation(locationData)
        addOutput(`Location acquired: ${formatCoordinate(locationData.latitude)}, ${formatCoordinate(locationData.longitude)}`)
        addOutput(`Accuracy: ${locationData.accuracy.toFixed(2)} meters`)

        if (locationData.altitude !== null) {
          addOutput(`Altitude: ${locationData.altitude.toFixed(2)} meters`)
        }
      },
      (error) => {
        let errorMsg = 'Unknown error'
        switch (error.code) {
          case error.PERMISSION_DENIED:
            errorMsg = 'Location permission denied by user'
            break
          case error.POSITION_UNAVAILABLE:
            errorMsg = 'Location information is unavailable'
            break
          case error.TIMEOUT:
            errorMsg = 'Location request timed out'
            break
        }
        setLocationError(errorMsg)
        addOutput(`Failed to get location: ${errorMsg}`, false)
      },
      {
        enableHighAccuracy: true,
        timeout: 10000,
        maximumAge: 0,
      }
    )
  }

  // Watch position (continuous tracking)
  const handleWatchLocation = () => {
    if (!checkGeolocationSupport()) return

    if (isWatchingLocation) {
      // Stop watching
      if (watchIdRef.current !== null) {
        navigator.geolocation.clearWatch(watchIdRef.current)
        watchIdRef.current = null
      }
      setIsWatchingLocation(false)
      addOutput('Stopped watching location')
      return
    }

    // Start watching
    addOutput('Starting continuous location tracking...')
    setLocationError(null)

    const watchId = navigator.geolocation.watchPosition(
      (position) => {
        const locationData: LocationData = {
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
          accuracy: position.coords.accuracy,
          altitude: position.coords.altitude,
          heading: position.coords.heading,
          speed: position.coords.speed,
          timestamp: position.timestamp,
        }

        setLocation(locationData)
        addOutput(`Location update: ${formatCoordinate(locationData.latitude)}, ${formatCoordinate(locationData.longitude)}`)
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
            errorMsg = 'Location timeout'
            break
        }
        setLocationError(errorMsg)
        addOutput(`Location error: ${errorMsg}`, false)
      },
      {
        enableHighAccuracy: true,
        timeout: 10000,
        maximumAge: 5000,
      }
    )

    watchIdRef.current = watchId
    setIsWatchingLocation(true)
  }

  // Simulate accelerometer data (for desktop)
  const handleSimulateMotion = () => {
    addOutput('Simulating motion sensor data (desktop mode)...')

    const simulatedData: SensorData = {
      x: Math.random() * 20 - 10,
      y: Math.random() * 20 - 10,
      z: Math.random() * 20 - 10,
      timestamp: Date.now(),
    }

    setAccelerometerData(simulatedData)
    addOutput(`Simulated acceleration: X=${simulatedData.x.toFixed(2)}, Y=${simulatedData.y.toFixed(2)}, Z=${simulatedData.z.toFixed(2)}`)
  }

  // Simulate proximity sensor
  const handleSimulateProximity = () => {
    addOutput('Simulating proximity sensor data...')
    const isNear = Math.random() > 0.5
    const simulated: ProximityData = {
      isNear,
      distance: isNear ? 0 : 5,
      maxRange: 5,
      timestamp: Date.now(),
    }
    setProximityData(simulated)
    addOutput(`Proximity: ${isNear ? 'Object detected nearby' : 'No object detected'}`)
  }

  // Simulate light sensor
  const handleSimulateLight = () => {
    addOutput('Simulating ambient light sensor...')
    const simulated: LightData = {
      illuminance: Math.random() * 1000, // 0-1000 lux
      timestamp: Date.now(),
    }
    setLightData(simulated)
    addOutput(`Light level: ${simulated.illuminance.toFixed(2)} lux`)
  }

  // Simulate pressure/barometer
  const handleSimulatePressure = () => {
    addOutput('Simulating pressure sensor...')
    const pressure = 1013 + Math.random() * 20 - 10 // Around sea level pressure
    const simulated: PressureData = {
      pressure,
      altitude: null,
      timestamp: Date.now(),
    }
    setPressureData(simulated)
    addOutput(`Atmospheric pressure: ${pressure.toFixed(2)} hPa`)
  }

  // Simulate temperature sensor
  const handleSimulateTemperature = () => {
    addOutput('Simulating temperature sensor...')
    const celsius = 20 + Math.random() * 10 // 20-30°C
    const simulated: TemperatureData = {
      celsius,
      fahrenheit: celsius * 9/5 + 32,
      type: 'ambient',
      timestamp: Date.now(),
    }
    setTemperatureData(simulated)
    addOutput(`Temperature: ${celsius.toFixed(1)}°C (${simulated.fahrenheit.toFixed(1)}°F)`)
  }

  // Simulate humidity sensor
  const handleSimulateHumidity = () => {
    addOutput('Simulating humidity sensor...')
    const simulated: HumidityData = {
      relativeHumidity: 30 + Math.random() * 40, // 30-70%
      timestamp: Date.now(),
    }
    setHumidityData(simulated)
    addOutput(`Relative humidity: ${simulated.relativeHumidity.toFixed(1)}%`)
  }

  // Simulate step counter
  const handleSimulateSteps = () => {
    addOutput('Simulating step counter...')
    const simulated: StepCounterData = {
      steps: Math.floor(Math.random() * 10000),
      timestamp: Date.now(),
    }
    setStepCounterData(simulated)
    addOutput(`Steps: ${simulated.steps}`)
  }

  // Clear all data
  const handleClearData = () => {
    setLocation(null)
    setAccelerometerData(null)
    setProximityData(null)
    setLightData(null)
    setPressureData(null)
    setTemperatureData(null)
    setHumidityData(null)
    setStepCounterData(null)
    setLocationError(null)
    addOutput('Cleared all sensor data')
  }

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (watchIdRef.current !== null) {
        navigator.geolocation.clearWatch(watchIdRef.current)
      }
    }
  }, [])

  return (
    <ModulePageLayout
      title="Sensors & Device Hardware Module"
      description="Access device sensors for motion tracking, compass, and GPS location"
      icon={Activity}
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
                <strong className="text-green-600">✓ Geolocation API</strong> - Web API for GPS location
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Motion Sensors</strong> - Accelerometer, Gyroscope, Magnetometer, Step Counter
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Environmental Sensors</strong> - Proximity, Light, Pressure, Temperature, Humidity
              </li>
              <li>
                <strong className="text-blue-600">ℹ Android</strong> - All sensors via SensorManager API
              </li>
              <li>
                <strong className="text-blue-600">ℹ iOS</strong> - CoreMotion, UIDevice, CMAltimeter, CMPedometer
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># For full sensor support, implement native plugins:</div>
              <div>Android: SensorManager (Motion + Environmental sensors)</div>
              <div>iOS: CoreMotion, UIDevice, CMAltimeter, CMPedometer</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Desktop: Geolocation API functional. Mobile sensors require custom plugin development.
            </p>
          </div>
        </section>

        {/* Geolocation Controls */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <MapPin className="w-5 h-5" />
            GPS / Geolocation
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Access device location using the Web Geolocation API
            </p>

            <div className="flex flex-wrap gap-2">
              <Button onClick={handleGetCurrentLocation} variant="outline">
                <MapPin className="w-4 h-4 mr-2" />
                Get Current Location
              </Button>

              <Button
                onClick={handleWatchLocation}
                variant={isWatchingLocation ? 'destructive' : 'outline'}
              >
                <Radio className={`w-4 h-4 mr-2 ${isWatchingLocation ? 'animate-pulse' : ''}`} />
                {isWatchingLocation ? 'Stop Tracking' : 'Watch Position'}
              </Button>

              <Button onClick={handleClearData} variant="ghost" size="sm">
                Clear Data
              </Button>
            </div>

            {locationError && (
              <div className="bg-destructive/10 border border-destructive/30 rounded-md p-3 text-sm text-destructive">
                {locationError}
              </div>
            )}
          </div>
        </section>

        {/* Location Data Display */}
        {location && (
          <section className="rounded-lg border p-6 space-y-4">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <MapPin className="w-5 h-5 text-green-600" />
              Location Data
            </h2>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-muted rounded-md p-4">
                <h4 className="font-semibold text-sm mb-2 text-muted-foreground">Coordinates</h4>
                <div className="space-y-1 font-mono text-sm">
                  <div>
                    <span className="text-muted-foreground">Latitude:</span>{' '}
                    <span className="font-semibold">{formatCoordinate(location.latitude)}</span>
                  </div>
                  <div>
                    <span className="text-muted-foreground">Longitude:</span>{' '}
                    <span className="font-semibold">{formatCoordinate(location.longitude)}</span>
                  </div>
                </div>
              </div>

              <div className="bg-muted rounded-md p-4">
                <h4 className="font-semibold text-sm mb-2 text-muted-foreground">Accuracy & Metrics</h4>
                <div className="space-y-1 font-mono text-sm">
                  <div>
                    <span className="text-muted-foreground">Accuracy:</span>{' '}
                    <span className="font-semibold">{location.accuracy.toFixed(2)}m</span>
                  </div>
                  <div>
                    <span className="text-muted-foreground">Altitude:</span>{' '}
                    <span className="font-semibold">
                      {location.altitude !== null ? `${location.altitude.toFixed(2)}m` : 'N/A'}
                    </span>
                  </div>
                </div>
              </div>

              <div className="bg-muted rounded-md p-4">
                <h4 className="font-semibold text-sm mb-2 text-muted-foreground">Movement</h4>
                <div className="space-y-1 font-mono text-sm">
                  <div>
                    <span className="text-muted-foreground">Speed:</span>{' '}
                    <span className="font-semibold">
                      {location.speed !== null ? `${location.speed.toFixed(2)} m/s` : 'N/A'}
                    </span>
                  </div>
                  <div>
                    <span className="text-muted-foreground">Heading:</span>{' '}
                    <span className="font-semibold">
                      {location.heading !== null ? `${location.heading.toFixed(2)}°` : 'N/A'}
                    </span>
                  </div>
                </div>
              </div>

              <div className="bg-muted rounded-md p-4">
                <h4 className="font-semibold text-sm mb-2 text-muted-foreground">Timestamp</h4>
                <div className="space-y-1 font-mono text-sm">
                  <div>
                    <span className="font-semibold">{new Date(location.timestamp).toLocaleString()}</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Map Placeholder */}
            <div className="bg-muted/30 rounded-md p-8 text-center border-2 border-dashed">
              <MapPin className="w-12 h-12 mx-auto mb-3 text-muted-foreground" />
              <p className="text-sm text-muted-foreground">
                Map integration coming soon (Leaflet.js)
              </p>
              <p className="text-xs text-muted-foreground mt-2">
                Install: <code className="bg-muted px-2 py-1 rounded">bun add leaflet</code>
              </p>
            </div>
          </section>
        )}

        {/* Motion Sensors */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Activity className="w-5 h-5" />
            Motion Sensors
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Access accelerometer, gyroscope, and magnetometer data
            </p>

            <div className="flex flex-wrap gap-2">
              <Button onClick={handleSimulateMotion} variant="outline">
                <Activity className="w-4 h-4 mr-2" />
                Simulate Motion (Desktop)
              </Button>
            </div>

            {accelerometerData && (
              <div className="grid grid-cols-3 gap-4 mt-4">
                <div className="bg-muted rounded-md p-4 text-center">
                  <div className="text-2xl font-bold text-red-600">{accelerometerData.x.toFixed(2)}</div>
                  <div className="text-sm text-muted-foreground mt-1">X-Axis</div>
                </div>
                <div className="bg-muted rounded-md p-4 text-center">
                  <div className="text-2xl font-bold text-green-600">{accelerometerData.y.toFixed(2)}</div>
                  <div className="text-sm text-muted-foreground mt-1">Y-Axis</div>
                </div>
                <div className="bg-muted rounded-md p-4 text-center">
                  <div className="text-2xl font-bold text-blue-600">{accelerometerData.z.toFixed(2)}</div>
                  <div className="text-sm text-muted-foreground mt-1">Z-Axis</div>
                </div>
              </div>
            )}

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400 text-sm">
                Mobile Sensor Implementation Required
              </h4>
              <p className="text-xs text-muted-foreground">
                Full sensor access requires custom native plugins for Android (SensorManager) and iOS (CoreMotion).
                Desktop simulation is shown for demonstration purposes.
              </p>
            </div>
          </div>
        </section>

        {/* Compass */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Compass className="w-5 h-5" />
            Compass / Magnetometer
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Device compass heading using magnetometer sensor
            </p>

            <div className="bg-muted/30 rounded-md p-8 text-center border-2 border-dashed">
              <Compass className="w-16 h-16 mx-auto mb-3 text-muted-foreground" />
              <p className="text-sm text-muted-foreground">
                Compass visualization requires custom plugin
              </p>
              <p className="text-xs text-muted-foreground mt-2">
                Android: TYPE_MAGNETIC_FIELD sensor
                <br />
                iOS: CMMotionManager magnetometer
              </p>
            </div>
          </div>
        </section>

        {/* Environmental Sensors */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Gauge className="w-5 h-5" />
            Environmental Sensors
          </h2>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {/* Proximity Sensor */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Eye className="w-5 h-5 text-purple-600" />
                <h3 className="font-semibold">Proximity</h3>
              </div>
              <Button onClick={handleSimulateProximity} variant="outline" size="sm" className="w-full">
                Simulate
              </Button>
              {proximityData && (
                <div className="bg-muted rounded-md p-3 text-sm space-y-1">
                  <div>Status: <span className="font-semibold">{proximityData.isNear ? 'Near' : 'Far'}</span></div>
                  <div>Distance: <span className="font-mono">{proximityData.distance !== null ? `${proximityData.distance} cm` : 'N/A'}</span></div>
                </div>
              )}
            </div>

            {/* Light Sensor */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Activity className="w-5 h-5 text-yellow-600" />
                <h3 className="font-semibold">Light</h3>
              </div>
              <Button onClick={handleSimulateLight} variant="outline" size="sm" className="w-full">
                Simulate
              </Button>
              {lightData && (
                <div className="bg-muted rounded-md p-3 text-sm">
                  <div className="font-mono text-lg text-center">{lightData.illuminance.toFixed(1)} lux</div>
                </div>
              )}
            </div>

            {/* Pressure Sensor */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Gauge className="w-5 h-5 text-blue-600" />
                <h3 className="font-semibold">Pressure</h3>
              </div>
              <Button onClick={handleSimulatePressure} variant="outline" size="sm" className="w-full">
                Simulate
              </Button>
              {pressureData && (
                <div className="bg-muted rounded-md p-3 text-sm">
                  <div className="font-mono text-lg text-center">{pressureData.pressure.toFixed(1)} hPa</div>
                </div>
              )}
            </div>

            {/* Temperature Sensor */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Thermometer className="w-5 h-5 text-red-600" />
                <h3 className="font-semibold">Temperature</h3>
              </div>
              <Button onClick={handleSimulateTemperature} variant="outline" size="sm" className="w-full">
                Simulate
              </Button>
              {temperatureData && (
                <div className="bg-muted rounded-md p-3 text-sm space-y-1">
                  <div className="font-mono text-lg text-center">{temperatureData.celsius.toFixed(1)}°C</div>
                  <div className="text-muted-foreground text-xs text-center">{temperatureData.fahrenheit.toFixed(1)}°F</div>
                </div>
              )}
            </div>

            {/* Humidity Sensor */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Droplets className="w-5 h-5 text-cyan-600" />
                <h3 className="font-semibold">Humidity</h3>
              </div>
              <Button onClick={handleSimulateHumidity} variant="outline" size="sm" className="w-full">
                Simulate
              </Button>
              {humidityData && (
                <div className="bg-muted rounded-md p-3 text-sm">
                  <div className="font-mono text-lg text-center">{humidityData.relativeHumidity.toFixed(1)}%</div>
                </div>
              )}
            </div>

            {/* Step Counter */}
            <div className="border rounded-lg p-4 space-y-3">
              <div className="flex items-center gap-2">
                <Footprints className="w-5 h-5 text-green-600" />
                <h3 className="font-semibold">Step Counter</h3>
              </div>
              <Button onClick={handleSimulateSteps} variant="outline" size="sm" className="w-full">
                Simulate
              </Button>
              {stepCounterData && (
                <div className="bg-muted rounded-md p-3 text-sm">
                  <div className="font-mono text-lg text-center">{stepCounterData.steps.toLocaleString()} steps</div>
                </div>
              )}
            </div>
          </div>

          <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
            <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400 text-sm">
              Mobile Implementation Required
            </h4>
            <p className="text-xs text-muted-foreground">
              Environmental sensors require custom native plugins. Android provides TYPE_PROXIMITY, TYPE_LIGHT,
              TYPE_PRESSURE, TYPE_AMBIENT_TEMPERATURE, and TYPE_RELATIVE_HUMIDITY sensors. iOS provides proximity
              via UIDevice and pressure via CMAltimeter, but no public APIs for light, temperature, or humidity.
            </p>
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
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Implementation Guide</h3>
          <div className="space-y-4 text-sm">
            <div className="space-y-2">
              <h4 className="font-semibold">Geolocation API (All Platforms)</h4>
              <p className="text-muted-foreground">
                Uses the Web Geolocation API which works on all platforms with proper permissions.
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>navigator.geolocation.getCurrentPosition(success, error, options)</div>
                <div>navigator.geolocation.watchPosition(success, error, options)</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Android Sensors</h4>
              <p className="text-muted-foreground">
                Requires custom plugin using Android SensorManager API
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>SensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)</div>
                <div>SensorManager.getDefaultSensor(Sensor.TYPE_GYROSCOPE)</div>
                <div>SensorManager.getDefaultSensor(Sensor.TYPE_MAGNETIC_FIELD)</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">iOS Sensors</h4>
              <p className="text-muted-foreground">
                Requires custom plugin using CoreMotion framework
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs">
                <div>let motionManager = CMMotionManager()</div>
                <div>motionManager.startAccelerometerUpdates()</div>
                <div>motionManager.startGyroUpdates()</div>
                <div>motionManager.startMagnetometerUpdates()</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Permissions & Privacy
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Request location permissions at runtime</li>
                <li>Explain why location access is needed</li>
                <li>Handle permission denial gracefully</li>
                <li>Use enableHighAccuracy only when needed</li>
                <li>Clear watch positions when done</li>
                <li>Consider battery impact</li>
              </ul>
            </div>
          </div>
        </section>

        {/* Platform Support */}
        <section className="rounded-lg border border-purple-500/50 bg-purple-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Platform Support</h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-2 px-4">Feature</th>
                  <th className="text-center py-2 px-4">Windows</th>
                  <th className="text-center py-2 px-4">macOS</th>
                  <th className="text-center py-2 px-4">Linux</th>
                  <th className="text-center py-2 px-4">iOS</th>
                  <th className="text-center py-2 px-4">Android</th>
                </tr>
              </thead>
              <tbody className="text-muted-foreground">
                <tr className="border-b bg-muted/30">
                  <td className="py-2 px-4 font-semibold" colSpan={6}>GPS & Location</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Geolocation API</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b bg-muted/30">
                  <td className="py-2 px-4 font-semibold" colSpan={6}>Motion Sensors</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Accelerometer</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌*</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Gyroscope</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Magnetometer/Compass</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Step Counter</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b bg-muted/30">
                  <td className="py-2 px-4 font-semibold" colSpan={6}>Environmental Sensors</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Proximity</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Ambient Light</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌***</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Barometer/Pressure</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Temperature</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌***</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
                <tr>
                  <td className="py-2 px-4">Humidity</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌</td>
                  <td className="text-center py-2 px-4">❌***</td>
                  <td className="text-center py-2 px-4">🔶**</td>
                </tr>
              </tbody>
            </table>
            <div className="text-xs text-muted-foreground mt-2 space-y-1">
              <p>* Some MacBooks have accelerometers (SMC sensor)</p>
              <p>** 🔶 = Requires custom plugin development</p>
              <p>*** iOS doesn't provide public APIs for ambient light, temperature, or humidity</p>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
