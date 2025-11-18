import { createFileRoute } from '@tanstack/react-router'
import { DollarSign, ShoppingCart, RefreshCw, Receipt } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState } from 'react'

export const Route = createFileRoute('/in-app-purchases')({
  component: InAppPurchasesModule,
})

interface Product {
  id: string
  title: string
  description: string
  price: string
  type: 'consumable' | 'non-consumable' | 'subscription'
}

interface PurchaseReceipt {
  productId: string
  transactionId: string
  purchaseDate: string
  status: 'pending' | 'completed' | 'failed'
}

function InAppPurchasesModule() {
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)
  const [selectedProduct, setSelectedProduct] = useState<string | null>(null)
  const [receipts, setReceipts] = useState<PurchaseReceipt[]>([])

  // Mock products for demonstration
  const [products] = useState<Product[]>([
    {
      id: 'premium_monthly',
      title: 'Premium Monthly',
      description: 'Access to all premium features for 1 month',
      price: '$9.99',
      type: 'subscription',
    },
    {
      id: 'premium_yearly',
      title: 'Premium Yearly',
      description: 'Access to all premium features for 1 year (save 20%)',
      price: '$99.99',
      type: 'subscription',
    },
    {
      id: 'coins_100',
      title: '100 Coins',
      description: 'Purchase 100 in-app coins',
      price: '$4.99',
      type: 'consumable',
    },
    {
      id: 'coins_500',
      title: '500 Coins',
      description: 'Purchase 500 in-app coins (best value)',
      price: '$19.99',
      type: 'consumable',
    },
    {
      id: 'remove_ads',
      title: 'Remove Ads',
      description: 'Permanently remove all advertisements',
      price: '$2.99',
      type: 'non-consumable',
    },
  ])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  const handleFetchProducts = async () => {
    setLoading('fetching')
    addOutput('Fetching available products...')

    try {
      // In a real implementation, this would call:
      // const products = await invoke('fetch_iap_products')

      await new Promise((resolve) => setTimeout(resolve, 1000))
      addOutput(`Fetched ${products.length} products successfully`)
      addOutput('Note: In-app purchase plugins not yet installed. This is a UI demonstration.')
    } catch (error) {
      addOutput(`Error fetching products: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handlePurchase = async (productId: string) => {
    const product = products.find((p) => p.id === productId)
    if (!product) return

    setLoading(`purchase-${productId}`)
    setSelectedProduct(productId)
    addOutput(`Initiating purchase for "${product.title}" (${product.price})...`)

    try {
      // In a real implementation, this would call:
      // const receipt = await invoke('purchase_product', { productId })

      await new Promise((resolve) => setTimeout(resolve, 2000))

      const mockReceipt: PurchaseReceipt = {
        productId,
        transactionId: `txn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        purchaseDate: new Date().toISOString(),
        status: 'completed',
      }

      setReceipts((prev) => [mockReceipt, ...prev])
      addOutput(`Purchase successful! Transaction ID: ${mockReceipt.transactionId}`)
      addOutput('Note: This is a mock purchase. Real IAP requires platform-specific plugin configuration.')
    } catch (error) {
      addOutput(`Purchase failed: ${error}`, false)
    } finally {
      setLoading(null)
      setSelectedProduct(null)
    }
  }

  const handleRestorePurchases = async () => {
    setLoading('restoring')
    addOutput('Restoring previous purchases...')

    try {
      // In a real implementation, this would call:
      // const restoredPurchases = await invoke('restore_purchases')

      await new Promise((resolve) => setTimeout(resolve, 1500))

      const restoredCount = receipts.filter((r) => r.status === 'completed').length
      addOutput(`Restored ${restoredCount} purchase(s)`)
      addOutput('Note: Real restore would query the platform store (App Store, Play Store, etc.)')
    } catch (error) {
      addOutput(`Restore failed: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleValidateReceipt = async (transactionId: string) => {
    addOutput(`Validating receipt: ${transactionId}...`)

    try {
      // In a real implementation, this would call:
      // const isValid = await invoke('validate_receipt', { transactionId })

      await new Promise((resolve) => setTimeout(resolve, 800))
      addOutput(`Receipt ${transactionId} is valid ✓`)
    } catch (error) {
      addOutput(`Validation failed: ${error}`, false)
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
        {/* Setup Notice */}
        <section className="rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-yellow-500">⚠️</span>
            Setup Required
          </h3>
          <div className="space-y-2 text-sm">
            <p>This module demonstrates the In-App Purchase UI and workflow.</p>
            <p className="font-medium">To enable real purchases, install the required plugins:</p>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># iOS + Android + Windows</div>
              <div>bun add tauri-plugin-in-app-purchase</div>
              <div className="mt-2"># Or iOS-specific</div>
              <div>bun add tauri-plugin-iap</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Platform-specific configuration is required in <code>src-tauri/capabilities/</code>
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
              No purchases yet. Try buying a product above.
            </p>
          ) : (
            <div className="space-y-3">
              {receipts.map((receipt, index) => {
                const product = products.find((p) => p.id === receipt.productId)
                return (
                  <div
                    key={index}
                    className="flex items-start justify-between p-4 border rounded-lg bg-muted/30"
                  >
                    <div className="flex-1 space-y-1">
                      <h3 className="font-semibold">{product?.title || receipt.productId}</h3>
                      <p className="text-xs text-muted-foreground">
                        Transaction ID: {receipt.transactionId}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Date: {new Date(receipt.purchaseDate).toLocaleString()}
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
                      onClick={() => handleValidateReceipt(receipt.transactionId)}
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
