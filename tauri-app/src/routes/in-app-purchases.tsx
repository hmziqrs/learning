import { createFileRoute } from '@tanstack/react-router'
import { DollarSign, ShoppingCart, RefreshCw, Receipt, Laptop } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

export const Route = createFileRoute('/in-app-purchases')({
  component: InAppPurchasesModule,
})

interface Product {
  id: string
  title: string
  description: string
  price: string
  price_amount: number
  currency: string
  type: 'consumable' | 'non-consumable' | 'subscription'
}

interface PurchaseReceipt {
  product_id: string
  transaction_id: string
  purchase_date: string
  status: string
  platform: string
  price_paid: number
  currency: string
}

interface RestoreResult {
  restored_count: number
  receipts: PurchaseReceipt[]
}

function InAppPurchasesModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [selectedProduct, setSelectedProduct] = useState<string | null>(null)
  const [receipts, setReceipts] = useState<PurchaseReceipt[]>([])
  const [products, setProducts] = useState<Product[]>([])
  const [platform, setPlatform] = useState<string>('unknown')

  useEffect(() => {
    loadInitialData()
  }, [])

  const loadInitialData = async () => {
    try {
      const platformName = await invoke<string>('get_iap_platform')
      setPlatform(platformName)
      addOutput(`Running on platform: ${platformName}`)
      addOutput('Click "Fetch Products" to load available products')
    } catch (error) {
      addOutput(`Error loading platform: ${error}`, false)
    }
  }

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const handleFetchProducts = async () => {
    setLoading('fetching')
    addOutput('Fetching available products from backend...')

    try {
      const fetchedProducts = await invoke<Product[]>('fetch_iap_products')
      setProducts(fetchedProducts)
      addOutput(`✓ Fetched ${fetchedProducts.length} products successfully`)
      addOutput('Note: Using mock/sandbox implementation. Products are simulated.')
    } catch (error) {
      addOutput(`✗ Error fetching products: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handlePurchase = async (productId: string) => {
    const product = products.find((p) => p.id === productId)
    if (!product) return

    setLoading(`purchase-${productId}`)
    setSelectedProduct(productId)
    addOutput(`💳 Initiating purchase for "${product.title}" (${product.price})...`)
    addOutput('Processing payment (simulated)...')

    try {
      const receipt = await invoke<PurchaseReceipt>('purchase_product', { productId })

      setReceipts((prev) => [receipt, ...prev])
      addOutput(`✓ Purchase successful!`)
      addOutput(`  Transaction ID: ${receipt.transaction_id}`)
      addOutput(`  Amount: ${receipt.currency} ${receipt.price_paid.toFixed(2)}`)
      addOutput(`  Platform: ${receipt.platform}`)
      addOutput('Note: This is a sandbox purchase. No real money was charged.')
    } catch (error) {
      addOutput(`✗ Purchase failed: ${error}`, false)
    } finally {
      setLoading(null)
      setSelectedProduct(null)
    }
  }

  const handleRestorePurchases = async () => {
    setLoading('restoring')
    addOutput('🔄 Restoring previous purchases...')
    addOutput('Querying platform store (simulated)...')

    try {
      const result = await invoke<RestoreResult>('restore_purchases')

      // Add restored receipts to the list
      if (result.receipts.length > 0) {
        setReceipts((prev) => [...result.receipts, ...prev])
        addOutput(`✓ Restored ${result.restored_count} purchase(s)`)
        result.receipts.forEach((receipt) => {
          addOutput(`  - ${receipt.product_id} (${receipt.transaction_id})`)
        })
      } else {
        addOutput('No previous purchases found to restore')
      }
      addOutput('Note: Real restore would query App Store/Play Store purchase history')
    } catch (error) {
      addOutput(`✗ Restore failed: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleValidateReceipt = async (transactionId: string) => {
    addOutput(`🔍 Validating receipt: ${transactionId}...`)
    addOutput('Contacting verification server (simulated)...')

    try {
      const isValid = await invoke<boolean>('validate_receipt', { transactionId })

      if (isValid) {
        addOutput(`✓ Receipt ${transactionId} is valid`)
        addOutput('Receipt verified successfully')
      } else {
        addOutput(`✗ Receipt ${transactionId} is invalid`, false)
      }
    } catch (error) {
      addOutput(`✗ Validation failed: ${error}`, false)
    }
  }

  const getProductTypeColor = (type: Product['type']) => {
    switch (type) {
      case 'subscription':
        return 'bg-purple-500/10 text-purple-500 border-purple-500/20'
      case 'consumable':
        return 'bg-blue-500/10 text-blue-500 border-blue-500/20'
      case 'non-consumable':
        return 'bg-green-500/10 text-green-500 border-green-500/20'
    }
  }

  return (
    <ModulePageLayout
      title="In-App Purchases Module"
      description="Test platform billing: iOS IAP, Android Billing, desktop Stripe"
      icon={DollarSign}
    >
      <div className="space-y-6">
        {/* Platform Info */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <Laptop className="w-5 h-5 text-blue-500" />
            Platform Information
          </h3>
          <div className="space-y-2 text-sm">
            <p>
              <strong>Current Platform:</strong>{' '}
              <span className="px-2 py-1 bg-muted rounded font-mono text-xs">{platform}</span>
            </p>
            <p className="text-muted-foreground">
              This module uses a mock/sandbox implementation for demonstration.
              Purchases are simulated and no real money is charged.
            </p>
          </div>
        </section>

        {/* Setup Notice */}
        <section className="rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-yellow-500">⚠️</span>
            Production Setup
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">To enable real purchases in production:</p>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Install platform-specific IAP plugin</div>
              <div>bun add tauri-plugin-in-app-purchase</div>
              <div className="mt-2"># Or for iOS-only</div>
              <div>bun add tauri-plugin-iap</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Additional setup required: App Store Connect (iOS), Google Play Console (Android),
              and platform-specific configuration in <code>src-tauri/</code>
            </p>
          </div>
        </section>

        {/* Actions */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <ShoppingCart className="w-5 h-5" />
            Quick Actions
          </h2>

          <div className="flex flex-wrap gap-3">
            <Button
              onClick={handleFetchProducts}
              disabled={loading === 'fetching'}
              variant="outline"
            >
              <RefreshCw className={`w-4 h-4 mr-2 ${loading === 'fetching' ? 'animate-spin' : ''}`} />
              {loading === 'fetching' ? 'Fetching...' : 'Fetch Products'}
            </Button>

            <Button
              onClick={handleRestorePurchases}
              disabled={loading === 'restoring'}
              variant="outline"
            >
              <Receipt className="w-4 h-4 mr-2" />
              {loading === 'restoring' ? 'Restoring...' : 'Restore Purchases'}
            </Button>
          </div>
        </section>

        {/* Products List */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <ShoppingCart className="w-5 h-5" />
            Available Products
          </h2>

          <div className="grid gap-4 md:grid-cols-2">
            {products.map((product) => (
              <div
                key={product.id}
                className="rounded-lg border p-4 space-y-3 hover:shadow-md transition-shadow"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <h3 className="font-semibold text-lg">{product.title}</h3>
                    <p className="text-sm text-muted-foreground mt-1">
                      {product.description}
                    </p>
                  </div>
                  <div className="text-right">
                    <div className="text-xl font-bold text-primary">{product.price}</div>
                  </div>
                </div>

                <div className="flex items-center justify-between">
                  <span
                    className={`text-xs px-2 py-1 rounded-full border ${getProductTypeColor(product.type)}`}
                  >
                    {product.type}
                  </span>

                  <Button
                    onClick={() => handlePurchase(product.id)}
                    disabled={loading === `purchase-${product.id}`}
                    size="sm"
                  >
                    {loading === `purchase-${product.id}` ? 'Processing...' : 'Buy Now'}
                  </Button>
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Purchase Receipts */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Receipt className="w-5 h-5" />
            Purchase Receipts ({receipts.length})
          </h2>

          {receipts.length === 0 ? (
            <p className="text-center text-muted-foreground py-8">
              No purchases yet. Try buying a product above or restore previous purchases.
            </p>
          ) : (
            <div className="space-y-3">
              {receipts.map((receipt, index) => {
                const product = products.find((p) => p.id === receipt.product_id)
                return (
                  <div
                    key={index}
                    className="flex items-start justify-between p-4 border rounded-lg bg-muted/30"
                  >
                    <div className="flex-1 space-y-1">
                      <h3 className="font-semibold">{product?.title || receipt.product_id}</h3>
                      <p className="text-xs text-muted-foreground">
                        Transaction ID: {receipt.transaction_id}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Date: {new Date(receipt.purchase_date).toLocaleString()}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Amount: {receipt.currency} {receipt.price_paid.toFixed(2)} • Platform: {receipt.platform}
                      </p>
                      <div className="flex items-center gap-2 mt-2">
                        <span
                          className={`text-xs px-2 py-1 rounded-full ${
                            receipt.status === 'completed'
                              ? 'bg-green-500/10 text-green-500'
                              : receipt.status === 'pending'
                              ? 'bg-yellow-500/10 text-yellow-500'
                              : 'bg-red-500/10 text-red-500'
                          }`}
                        >
                          {receipt.status}
                        </span>
                      </div>
                    </div>
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => handleValidateReceipt(receipt.transaction_id)}
                    >
                      Validate
                    </Button>
                  </div>
                )
              })}
            </div>
          )}
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

        {/* Implementation Notes */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Implementation Notes</h3>
          <div className="space-y-2 text-sm text-muted-foreground">
            <p>
              <strong>iOS (App Store):</strong> Requires Apple Developer account, StoreKit configuration,
              and app-specific product IDs created in App Store Connect.
            </p>
            <p>
              <strong>Android (Play Store):</strong> Requires Google Play Console setup, billing library
              integration, and product/subscription configuration.
            </p>
            <p>
              <strong>Desktop (Stripe):</strong> Can integrate Stripe Checkout or Payment Links for
              web-based payments in desktop builds.
            </p>
            <p className="mt-3">
              <strong>Plugin Setup:</strong> After installing the plugin, add Tauri commands in{' '}
              <code>src-tauri/src/lib.rs</code> to handle platform-specific purchase flows.
            </p>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
