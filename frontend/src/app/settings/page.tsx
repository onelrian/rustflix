'use client'

import React, { useState } from 'react'
import { useTheme } from 'next-themes'
import { Layout } from '@/components/layout/Layout'
import { Button } from '@/components/ui/Button'
import { Input } from '@/components/ui/Input'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card'
import { useAuth } from '@/contexts/AuthContext'
import { userApi } from '@/lib/api'
import { Settings, User, Palette, Video, Volume2 } from 'lucide-react'

export default function SettingsPage() {
  const { user } = useAuth()
  const { theme, setTheme } = useTheme()
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')

  const handlePreferenceUpdate = async (key: string, value: any) => {
    try {
      setLoading(true)
      await userApi.updatePreferences({ [key]: value })
      setMessage('Preferences updated successfully')
      setTimeout(() => setMessage(''), 3000)
    } catch (error) {
      setMessage('Failed to update preferences')
      setTimeout(() => setMessage(''), 3000)
    } finally {
      setLoading(false)
    }
  }

  if (!user) return null

  return (
    <Layout>
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="flex items-center space-x-3">
          <Settings className="h-8 w-8" />
          <h1 className="text-3xl font-bold">Settings</h1>
        </div>

        {message && (
          <div className="rounded-md bg-green-50 dark:bg-green-900/20 p-4">
            <div className="text-sm text-green-700 dark:text-green-400">{message}</div>
          </div>
        )}

        <div className="grid gap-6">
          {/* Profile Settings */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <User className="h-5 w-5" />
                <span>Profile</span>
              </CardTitle>
              <CardDescription>Manage your account information</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <Input
                  label="Username"
                  value={user.username}
                  disabled
                />
                <Input
                  label="Email"
                  value={user.email}
                  disabled
                />
              </div>
              <div className="flex items-center space-x-2">
                <span className="text-sm font-medium">Role:</span>
                <span className="capitalize px-2 py-1 bg-secondary text-secondary-foreground rounded text-sm">
                  {user.role}
                </span>
              </div>
            </CardContent>
          </Card>

          {/* Appearance Settings */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Palette className="h-5 w-5" />
                <span>Appearance</span>
              </CardTitle>
              <CardDescription>Customize the look and feel</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="text-sm font-medium mb-2 block">Theme</label>
                <div className="flex space-x-2">
                  <Button
                    variant={theme === 'light' ? 'primary' : 'outline'}
                    onClick={() => setTheme('light')}
                    size="sm"
                  >
                    Light
                  </Button>
                  <Button
                    variant={theme === 'dark' ? 'primary' : 'outline'}
                    onClick={() => setTheme('dark')}
                    size="sm"
                  >
                    Dark
                  </Button>
                  <Button
                    variant={theme === 'system' ? 'primary' : 'outline'}
                    onClick={() => setTheme('system')}
                    size="sm"
                  >
                    System
                  </Button>
                </div>
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Language</label>
                <select
                  value={user.preferences.language}
                  onChange={(e) => handlePreferenceUpdate('language', e.target.value)}
                  className="w-full p-2 border rounded-md bg-background"
                  disabled={loading}
                >
                  <option value="en">English</option>
                  <option value="es">Spanish</option>
                  <option value="fr">French</option>
                  <option value="de">German</option>
                </select>
              </div>
            </CardContent>
          </Card>

          {/* Playback Settings */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Video className="h-5 w-5" />
                <span>Playback</span>
              </CardTitle>
              <CardDescription>Configure video and audio preferences</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium">Autoplay</label>
                  <p className="text-xs text-muted-foreground">Automatically play next episode</p>
                </div>
                <input
                  type="checkbox"
                  checked={user.preferences.autoplay}
                  onChange={(e) => handlePreferenceUpdate('autoplay', e.target.checked)}
                  className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                  disabled={loading}
                />
              </div>

              <div className="flex items-center justify-between">
                <div>
                  <label className="text-sm font-medium">Subtitles</label>
                  <p className="text-xs text-muted-foreground">Show subtitles by default</p>
                </div>
                <input
                  type="checkbox"
                  checked={user.preferences.subtitles}
                  onChange={(e) => handlePreferenceUpdate('subtitles', e.target.checked)}
                  className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                  disabled={loading}
                />
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Default Quality</label>
                <select
                  value={user.preferences.quality}
                  onChange={(e) => handlePreferenceUpdate('quality', e.target.value)}
                  className="w-full p-2 border rounded-md bg-background"
                  disabled={loading}
                >
                  <option value="auto">Auto</option>
                  <option value="480p">480p</option>
                  <option value="720p">720p</option>
                  <option value="1080p">1080p</option>
                  <option value="4k">4K</option>
                </select>
              </div>
            </CardContent>
          </Card>

          {/* Account Actions */}
          <Card>
            <CardHeader>
              <CardTitle>Account Actions</CardTitle>
              <CardDescription>Manage your account</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <Button variant="outline" className="w-full">
                Change Password
              </Button>
              <Button variant="outline" className="w-full">
                Download Data
              </Button>
              <Button variant="destructive" className="w-full">
                Delete Account
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </Layout>
  )
}
