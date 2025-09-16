'use client'

import React from 'react'
import Link from 'next/link'
import Image from 'next/image'
import { Play, Plus, Check, Star } from 'lucide-react'
import { MediaItem } from '@/types'
import { formatDuration } from '@/lib/utils'
import { Button } from '@/components/ui/Button'
import { useAddToWatchlist, useRemoveFromWatchlist } from '@/hooks/useMediaQuery'

interface MediaCardProps {
  media: MediaItem
  inWatchlist?: boolean
}

export function MediaCard({ media, inWatchlist = false }: MediaCardProps) {
  const addToWatchlist = useAddToWatchlist()
  const removeFromWatchlist = useRemoveFromWatchlist()

  const handleWatchlistToggle = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    
    if (inWatchlist) {
      removeFromWatchlist.mutate(media.id)
    } else {
      addToWatchlist.mutate(media.id)
    }
  }

  const getMediaUrl = () => {
    switch (media.mediaType) {
      case 'tv_show':
        return `/tv/${media.id}`
      case 'music':
        return `/music/${media.id}`
      default:
        return `/movies/${media.id}`
    }
  }

  return (
    <div className="group relative overflow-hidden rounded-lg bg-card shadow-sm transition-all hover:shadow-lg">
      <Link href={getMediaUrl()}>
        <div className="aspect-[2/3] relative overflow-hidden">
          {media.metadata.poster ? (
            <Image
              src={media.metadata.poster}
              alt={media.title}
              fill
              className="object-cover transition-transform group-hover:scale-105"
              sizes="(max-width: 768px) 50vw, (max-width: 1200px) 33vw, 25vw"
            />
          ) : (
            <div className="w-full h-full bg-gray-200 dark:bg-gray-700 flex items-center justify-center">
              <span className="text-gray-400 text-sm">No Image</span>
            </div>
          )}
          
          {/* Overlay */}
          <div className="absolute inset-0 bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity">
            <div className="absolute inset-0 flex items-center justify-center">
              <Button size="sm" className="rounded-full">
                <Play className="h-4 w-4 mr-1" />
                Play
              </Button>
            </div>
          </div>

          {/* Rating */}
          {media.metadata.rating && (
            <div className="absolute top-2 left-2 flex items-center space-x-1 bg-black/70 rounded px-2 py-1">
              <Star className="h-3 w-3 text-yellow-400 fill-current" />
              <span className="text-xs text-white">{media.metadata.rating.toFixed(1)}</span>
            </div>
          )}

          {/* Watchlist Button */}
          <Button
            variant="ghost"
            size="sm"
            className="absolute top-2 right-2 rounded-full bg-black/70 hover:bg-black/80"
            onClick={handleWatchlistToggle}
          >
            {inWatchlist ? (
              <Check className="h-4 w-4 text-green-400" />
            ) : (
              <Plus className="h-4 w-4 text-white" />
            )}
          </Button>
        </div>
      </Link>

      <div className="p-4">
        <Link href={getMediaUrl()}>
          <h3 className="font-semibold text-sm mb-1 line-clamp-2 hover:text-primary">
            {media.title}
          </h3>
        </Link>
        
        <div className="flex items-center justify-between text-xs text-muted-foreground">
          <span className="capitalize">{media.mediaType.replace('_', ' ')}</span>
          {media.duration && (
            <span>{formatDuration(media.duration)}</span>
          )}
        </div>

        {media.metadata.releaseDate && (
          <p className="text-xs text-muted-foreground mt-1">
            {new Date(media.metadata.releaseDate).getFullYear()}
          </p>
        )}

        {media.metadata.genres.length > 0 && (
          <div className="flex flex-wrap gap-1 mt-2">
            {media.metadata.genres.slice(0, 2).map((genre) => (
              <span
                key={genre}
                className="inline-block bg-secondary text-secondary-foreground px-2 py-1 rounded text-xs"
              >
                {genre}
              </span>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
