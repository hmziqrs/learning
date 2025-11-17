/**
 * Global setup for E2E tests
 * This file runs once before all tests
 */

import { mkdirSync } from 'fs'
import { join } from 'path'

export default async function globalSetup() {
  console.log('🔧 Running global E2E test setup...')

  // Create screenshots directory if it doesn't exist
  try {
    mkdirSync(join(process.cwd(), 'e2e', 'screenshots'), { recursive: true })
    console.log('📁 Screenshots directory created')
  } catch (error) {
    // Directory might already exist
  }

  // You can add more global setup here:
  // - Start mock servers
  // - Set up test databases
  // - Initialize test data
  // - etc.

  console.log('✅ Global setup complete')
}
