import { createFileRoute } from '@tanstack/react-router'
import { Users, Check, X, Search, Phone, Mail, RefreshCw } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'

export const Route = createFileRoute('/contacts')({
  component: Contacts,
})

interface PhoneNumber {
  type: string
  number: string
}

interface Email {
  type: string
  address: string
}

interface Contact {
  id: string
  name: string
  phoneNumbers: PhoneNumber[]
  emails: Email[]
  photoUri?: string
}

function Contacts() {
  const [permissionGranted, setPermissionGranted] = useState<boolean | null>(null)
  const [contacts, setContacts] = useState<Contact[]>([])
  const [searchQuery, setSearchQuery] = useState('')
  const [output, setOutput] = useState<string[]>([])
  const [loading, setLoading] = useState<string | null>(null)

  useEffect(() => {
    checkPermission()
  }, [])

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    setOutput((prev) => [...prev, `${icon} ${message}`])
  }

  const checkPermission = async () => {
    try {
      const granted = await invoke<boolean>('check_contacts_permission')
      setPermissionGranted(granted)
      addOutput(`Permission status: ${granted ? 'Granted' : 'Not granted'}`, granted)
    } catch (error) {
      // Desktop or unsupported platform
      const errorMsg = String(error)
      if (errorMsg.includes('mobile platforms')) {
        setPermissionGranted(false)
        addOutput('ℹ Contacts API is only available on mobile platforms', false)
      } else {
        addOutput(`Error checking permission: ${error}`, false)
      }
    }
  }

  const handleRequestPermission = async () => {
    setLoading('permission')
    try {
      const granted = await invoke<boolean>('request_contacts_permission')
      setPermissionGranted(granted)
      addOutput(`Permission ${granted ? 'granted' : 'denied'}`, granted)
    } catch (error) {
      addOutput(`Error requesting permission: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const handleLoadContacts = async () => {
    if (!permissionGranted) {
      addOutput('Permission not granted. Please request permission first.', false)
      return
    }

    setLoading('load')
    try {
      const contactList = await invoke<Contact[]>('get_contacts')
      setContacts(contactList)
      addOutput(`Loaded ${contactList.length} contacts successfully`)
    } catch (error) {
      addOutput(`Error loading contacts: ${error}`, false)
    } finally {
      setLoading(null)
    }
  }

  const filteredContacts = contacts.filter((contact) => {
    const query = searchQuery.toLowerCase()
    return (
      contact.name.toLowerCase().includes(query) ||
      contact.phoneNumbers.some((p) => p.number.includes(query)) ||
      contact.emails.some((e) => e.address.toLowerCase().includes(query))
    )
  })

  const getPhoneTypeIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case 'mobile':
        return '📱'
      case 'home':
        return '🏠'
      case 'work':
        return '💼'
      default:
        return '📞'
    }
  }

  const getEmailTypeIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case 'personal':
        return '👤'
      case 'work':
        return '💼'
      default:
        return '📧'
    }
  }

  return (
    <ModulePageLayout
      title="Contacts Module"
      description="Access device contacts with read capabilities (mobile-only)."
      icon={Users}
    >
      <div className="space-y-6">
        {/* Permission Section */}
        <div className="space-y-4">
          <h3 className="font-semibold">Permission Status</h3>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              {permissionGranted === null ? (
                <span className="text-muted-foreground">Checking...</span>
              ) : permissionGranted ? (
                <>
                  <Check className="h-5 w-5 text-green-500" />
                  <span className="text-green-500">Granted</span>
                </>
              ) : (
                <>
                  <X className="h-5 w-5 text-red-500" />
                  <span className="text-red-500">Not Granted</span>
                </>
              )}
            </div>
            {permissionGranted === false && (
              <Button
                onClick={handleRequestPermission}
                disabled={loading === 'permission'}
              >
                Request Permission
              </Button>
            )}
          </div>
        </div>

        {/* Load Contacts Section */}
        <div className="space-y-4">
          <h3 className="font-semibold flex items-center gap-2">
            <Users className="h-5 w-5" />
            Load Contacts
          </h3>
          <div className="flex items-center gap-3">
            <Button
              onClick={handleLoadContacts}
              disabled={loading === 'load' || !permissionGranted}
            >
              <RefreshCw className={`h-4 w-4 mr-2 ${loading === 'load' ? 'animate-spin' : ''}`} />
              {loading === 'load' ? 'Loading...' : 'Load Contacts'}
            </Button>
            {contacts.length > 0 && (
              <span className="text-sm text-muted-foreground">
                Total: {contacts.length} contact{contacts.length !== 1 ? 's' : ''}
              </span>
            )}
          </div>
        </div>

        {/* Search Section */}
        {contacts.length > 0 && (
          <div className="space-y-4">
            <h3 className="font-semibold flex items-center gap-2">
              <Search className="h-5 w-5" />
              Search Contacts
            </h3>
            <div className="relative">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <input
                type="text"
                className="w-full pl-10 pr-3 py-2 border rounded-md"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search by name, phone, or email..."
              />
              {searchQuery && (
                <button
                  onClick={() => setSearchQuery('')}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                >
                  <X className="h-4 w-4" />
                </button>
              )}
            </div>
            {searchQuery && (
              <p className="text-sm text-muted-foreground">
                Found {filteredContacts.length} contact{filteredContacts.length !== 1 ? 's' : ''}
              </p>
            )}
          </div>
        )}

        {/* Contact List Section */}
        {contacts.length > 0 && (
          <div className="space-y-4">
            <h3 className="font-semibold">
              {searchQuery ? 'Search Results' : 'All Contacts'}
            </h3>
            <div className="space-y-3 max-h-[500px] overflow-y-auto">
              {filteredContacts.length === 0 ? (
                <div className="text-center py-8 text-muted-foreground">
                  No contacts found matching "{searchQuery}"
                </div>
              ) : (
                filteredContacts.map((contact) => (
                  <div
                    key={contact.id}
                    className="p-4 border rounded-lg bg-card hover:bg-muted/50 transition-colors"
                  >
                    <div className="flex items-start gap-3">
                      {/* Contact Avatar */}
                      <div className="flex-shrink-0 w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
                        <Users className="h-6 w-6 text-primary" />
                      </div>

                      {/* Contact Info */}
                      <div className="flex-1 min-w-0">
                        <h4 className="font-semibold text-lg mb-2">
                          {contact.name || 'Unknown'}
                        </h4>

                        {/* Phone Numbers */}
                        {contact.phoneNumbers.length > 0 && (
                          <div className="space-y-1 mb-2">
                            {contact.phoneNumbers.map((phone, index) => (
                              <div
                                key={index}
                                className="flex items-center gap-2 text-sm"
                              >
                                <Phone className="h-3 w-3 text-muted-foreground" />
                                <span className="text-muted-foreground">
                                  {getPhoneTypeIcon(phone.type)} {phone.type}:
                                </span>
                                <span className="font-mono">{phone.number}</span>
                              </div>
                            ))}
                          </div>
                        )}

                        {/* Email Addresses */}
                        {contact.emails.length > 0 && (
                          <div className="space-y-1">
                            {contact.emails.map((email, index) => (
                              <div
                                key={index}
                                className="flex items-center gap-2 text-sm"
                              >
                                <Mail className="h-3 w-3 text-muted-foreground" />
                                <span className="text-muted-foreground">
                                  {getEmailTypeIcon(email.type)} {email.type}:
                                </span>
                                <span className="break-all">{email.address}</span>
                              </div>
                            ))}
                          </div>
                        )}

                        {/* Empty State */}
                        {contact.phoneNumbers.length === 0 &&
                          contact.emails.length === 0 && (
                            <p className="text-sm text-muted-foreground italic">
                              No contact information available
                            </p>
                          )}
                      </div>
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>
        )}

        {/* Empty State */}
        {contacts.length === 0 && permissionGranted && (
          <div className="text-center py-12 border-2 border-dashed rounded-lg">
            <Users className="h-12 w-12 mx-auto text-muted-foreground mb-3" />
            <h3 className="font-semibold mb-1">No Contacts Loaded</h3>
            <p className="text-sm text-muted-foreground mb-4">
              Click "Load Contacts" to fetch contacts from your device
            </p>
          </div>
        )}

        {/* Output Panel */}
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <h3 className="font-semibold">Output</h3>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>
          <div className="bg-muted p-4 rounded-md font-mono text-sm min-h-[150px] max-h-[300px] overflow-y-auto">
            {output.length === 0 ? (
              <p className="text-muted-foreground">
                Operation results will appear here...
              </p>
            ) : (
              <div className="space-y-1">
                {output.map((line, index) => (
                  <div key={index}>{line}</div>
                ))}
              </div>
            )}
          </div>
        </div>
      </div>
    </ModulePageLayout>
  )
}
