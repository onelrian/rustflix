'use client'

import { useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { useAuth } from '@/contexts/AuthContext'
import { Layout } from '@/components/layout/Layout'
import { MediaGrid } from '@/components/media/MediaGrid'
import { useMedia, useWatchlist } from '@/hooks/useMediaQuery'
import { Button } from '@/components/ui/Button'
import { Play, TrendingUp, Clock } from 'lucide-react'

export default function Home() {
  const { isAuthenticated, loading } = useAuth()
  const router = useRouter()
  const { data: mediaData, isLoading: mediaLoading } = useMedia({ sortBy: 'releaseDate', sortOrder: 'desc' })
  const { data: watchlistData } = useWatchlist()

  useEffect(() => {
    if (!loading && !isAuthenticated) {
      router.push('/auth/login')
    }
  }, [isAuthenticated, loading, router])

  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    )
  }

  if (!isAuthenticated) {
    return null
  }

  const recentMedia = mediaData?.data.slice(0, 12) || []
  const watchlistIds = watchlistData?.data.map(item => item.id) || []

  return (
    <Layout>
      <div className="space-y-8">
        {/* Hero Section */}
        <section className="relative overflow-hidden rounded-lg bg-gradient-to-r from-blue-600 to-purple-600 p-8 text-white">
          <div className="relative z-10">
            <h1 className="text-4xl font-bold mb-4">Welcome to RustFlix</h1>
            <p className="text-xl mb-6 opacity-90">
              Discover and stream your favorite movies, TV shows, and music
            </p>
            <div className="flex flex-wrap gap-4">
              <Button size="lg" className="bg-white text-blue-600 hover:bg-gray-100">
                <Play className="h-5 w-5 mr-2" />
                Start Watching
              </Button>
              <Button variant="outline" size="lg" className="border-white text-white hover:bg-white hover:text-blue-600">
                Browse Library
              </Button>
            </div>
          </div>
        </section>

        {/* Quick Stats */}
        <section className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="bg-card rounded-lg p-6 border">
            <div className="flex items-center space-x-3">
              <TrendingUp className="h-8 w-8 text-blue-500" />
              <div>
                <p className="text-2xl font-bold">{mediaData?.pagination.total || 0}</p>
                <p className="text-muted-foreground">Total Media Items</p>
              </div>
            </div>
          </div>
          <div className="bg-card rounded-lg p-6 border">
            <div className="flex items-center space-x-3">
              <Clock className="h-8 w-8 text-green-500" />
              <div>
                <p className="text-2xl font-bold">{watchlistData?.pagination.total || 0}</p>
                <p className="text-muted-foreground">In Watchlist</p>
              </div>
            </div>
          </div>
          <div className="bg-card rounded-lg p-6 border">
            <div className="flex items-center space-x-3">
              <Play className="h-8 w-8 text-purple-500" />
              <div>
                <p className="text-2xl font-bold">0</p>
                <p className="text-muted-foreground">Hours Watched</p>
              </div>
            </div>
          </div>
        </section>

        {/* Recently Added */}
        <section>
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-2xl font-bold">Recently Added</h2>
            <Button variant="outline" onClick={() => router.push('/browse')}>
              View All
            </Button>
          </div>
          <MediaGrid 
            media={recentMedia} 
            watchlistIds={watchlistIds}
            loading={mediaLoading}
          />
        </section>

        {/* Continue Watching */}
        {watchlistData && watchlistData.data.length > 0 && (
          <section>
            <div className="flex items-center justify-between mb-6">
              <h2 className="text-2xl font-bold">Your Watchlist</h2>
              <Button variant="outline" onClick={() => router.push('/watchlist')}>
                View All
              </Button>
            </div>
            <MediaGrid 
              media={watchlistData.data.slice(0, 6)} 
              watchlistIds={watchlistIds}
            />
          </section>
        )}
      </div>
    </Layout>
  )
}
