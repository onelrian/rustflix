'use client'

import React from 'react'
import { Layout } from '@/components/layout/Layout'
import { MediaGrid } from '@/components/media/MediaGrid'
import { useWatchlist } from '@/hooks/useMediaQuery'
import { Heart } from 'lucide-react'

export default function WatchlistPage() {
  const { data: watchlistData, isLoading } = useWatchlist()

  const watchlistIds = watchlistData?.data.map(item => item.id) || []

  return (
    <Layout>
      <div className="space-y-6">
        <div className="flex items-center space-x-3">
          <Heart className="h-8 w-8 text-red-500" />
          <h1 className="text-3xl font-bold">My Watchlist</h1>
        </div>

        {watchlistData && watchlistData.data.length > 0 ? (
          <div>
            <p className="text-muted-foreground mb-6">
              {watchlistData.pagination.total} items in your watchlist
            </p>
            <MediaGrid
              media={watchlistData.data}
              watchlistIds={watchlistIds}
              loading={isLoading}
            />
          </div>
        ) : (
          <div className="text-center py-12">
            <Heart className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
            <h2 className="text-xl font-semibold mb-2">Your watchlist is empty</h2>
            <p className="text-muted-foreground">
              Add movies and TV shows to your watchlist to watch them later
            </p>
          </div>
        )}
      </div>
    </Layout>
  )
}
