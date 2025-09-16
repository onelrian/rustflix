'use client'

import React from 'react'
import { MediaItem } from '@/types'
import { MediaCard } from './MediaCard'

interface MediaGridProps {
  media: MediaItem[]
  watchlistIds?: string[]
  loading?: boolean
}

export function MediaGrid({ media, watchlistIds = [], loading = false }: MediaGridProps) {
  if (loading) {
    return (
      <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
        {Array.from({ length: 12 }).map((_, i) => (
          <div key={i} className="animate-pulse">
            <div className="aspect-[2/3] bg-gray-200 dark:bg-gray-700 rounded-lg mb-2" />
            <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded mb-1" />
            <div className="h-3 bg-gray-200 dark:bg-gray-700 rounded w-2/3" />
          </div>
        ))}
      </div>
    )
  }

  if (media.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">No media found.</p>
      </div>
    )
  }

  return (
    <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
      {media.map((item) => (
        <MediaCard
          key={item.id}
          media={item}
          inWatchlist={watchlistIds.includes(item.id)}
        />
      ))}
    </div>
  )
}
