'use client'

import React, { useState } from 'react'
import { useParams, useRouter } from 'next/navigation'
import Image from 'next/image'
import { Play, Plus, Check, Star, Calendar, Clock, Users } from 'lucide-react'
import { Layout } from '@/components/layout/Layout'
import { Button } from '@/components/ui/Button'
import { VideoPlayer } from '@/components/media/VideoPlayer'
import { Modal } from '@/components/ui/Modal'
import { useMediaById, useStreamUrl, usePlaybackState, useAddToWatchlist, useRemoveFromWatchlist, useWatchlist } from '@/hooks/useMediaQuery'
import { formatDuration, formatDate } from '@/lib/utils'

export default function MoviePage() {
  const params = useParams()
  const router = useRouter()
  const movieId = params.id as string
  const [isPlayerOpen, setIsPlayerOpen] = useState(false)
  
  const { data: movie, isLoading } = useMediaById(movieId)
  const { data: streamUrl } = useStreamUrl(movieId, 'hls')
  const { data: playbackState } = usePlaybackState(movieId)
  const { data: watchlistData } = useWatchlist()
  const addToWatchlist = useAddToWatchlist()
  const removeFromWatchlist = useRemoveFromWatchlist()

  const isInWatchlist = watchlistData?.data.some((item: { id: string }) => item.id === movieId) || false

  const handleWatchlistToggle = () => {
    if (isInWatchlist) {
      removeFromWatchlist.mutate(movieId)
    } else {
      addToWatchlist.mutate(movieId)
    }
  }

  const handlePlay = () => {
    setIsPlayerOpen(true)
  }

  if (isLoading) {
    return (
      <Layout>
        <div className="animate-pulse space-y-8">
          <div className="aspect-video bg-gray-200 dark:bg-gray-700 rounded-lg" />
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <div className="lg:col-span-2 space-y-4">
              <div className="h-8 bg-gray-200 dark:bg-gray-700 rounded w-3/4" />
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-full" />
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-5/6" />
            </div>
            <div className="space-y-4">
              <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/2" />
              <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded w-full" />
            </div>
          </div>
        </div>
      </Layout>
    )
  }

  if (!movie) {
    return (
      <Layout>
        <div className="text-center py-12">
          <h1 className="text-2xl font-bold mb-4">Movie not found</h1>
          <Button onClick={() => router.back()}>Go Back</Button>
        </div>
      </Layout>
    )
  }

  return (
    <Layout>
      <div className="space-y-8">
        {/* Hero Section */}
        <div className="relative aspect-video rounded-lg overflow-hidden bg-gray-900">
          {movie.metadata.backdrop ? (
            <Image
              src={movie.metadata.backdrop}
              alt={movie.title}
              fill
              className="object-cover opacity-60"
              priority
            />
          ) : (
            <div className="w-full h-full bg-gradient-to-r from-gray-800 to-gray-900" />
          )}
          
          <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent" />
          
          <div className="absolute bottom-0 left-0 right-0 p-8">
            <div className="max-w-4xl">
              <h1 className="text-4xl md:text-6xl font-bold text-white mb-4">
                {movie.title}
              </h1>
              
              <div className="flex flex-wrap items-center gap-4 text-white/80 mb-6">
                {movie.metadata.rating && (
                  <div className="flex items-center space-x-1">
                    <Star className="h-5 w-5 text-yellow-400 fill-current" />
                    <span>{movie.metadata.rating.toFixed(1)}</span>
                  </div>
                )}
                
                {movie.metadata.releaseDate && (
                  <div className="flex items-center space-x-1">
                    <Calendar className="h-5 w-5" />
                    <span>{new Date(movie.metadata.releaseDate).getFullYear()}</span>
                  </div>
                )}
                
                {movie.duration && (
                  <div className="flex items-center space-x-1">
                    <Clock className="h-5 w-5" />
                    <span>{formatDuration(movie.duration)}</span>
                  </div>
                )}
              </div>
              
              <div className="flex flex-wrap gap-4">
                <Button size="lg" onClick={handlePlay} className="bg-white text-black hover:bg-gray-200">
                  <Play className="h-5 w-5 mr-2" />
                  {playbackState?.position ? 'Continue Watching' : 'Play'}
                </Button>
                
                <Button
                  variant="outline"
                  size="lg"
                  onClick={handleWatchlistToggle}
                  className="border-white text-white hover:bg-white hover:text-black"
                >
                  {isInWatchlist ? (
                    <>
                      <Check className="h-5 w-5 mr-2" />
                      In Watchlist
                    </>
                  ) : (
                    <>
                      <Plus className="h-5 w-5 mr-2" />
                      Add to Watchlist
                    </>
                  )}
                </Button>
              </div>
            </div>
          </div>
        </div>

        {/* Content */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Main Content */}
          <div className="lg:col-span-2 space-y-6">
            {/* Description */}
            <div>
              <h2 className="text-2xl font-bold mb-4">Overview</h2>
              <p className="text-muted-foreground leading-relaxed">
                {movie.metadata.description || 'No description available.'}
              </p>
            </div>

            {/* Genres */}
            {movie.metadata.genres.length > 0 && (
              <div>
                <h3 className="text-lg font-semibold mb-3">Genres</h3>
                <div className="flex flex-wrap gap-2">
                  {movie.metadata.genres.map((genre) => (
                    <span
                      key={genre}
                      className="px-3 py-1 bg-secondary text-secondary-foreground rounded-full text-sm"
                    >
                      {genre}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Cast */}
            {movie.metadata.cast.length > 0 && (
              <div>
                <h3 className="text-lg font-semibold mb-3">Cast</h3>
                <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                  {movie.metadata.cast.slice(0, 8).map((person) => (
                    <div key={person.id} className="text-center">
                      {person.profileImage ? (
                        <Image
                          src={person.profileImage}
                          alt={person.name}
                          width={100}
                          height={150}
                          className="rounded-lg mx-auto mb-2"
                        />
                      ) : (
                        <div className="w-[100px] h-[150px] bg-gray-200 dark:bg-gray-700 rounded-lg mx-auto mb-2 flex items-center justify-center">
                          <Users className="h-8 w-8 text-gray-400" />
                        </div>
                      )}
                      <p className="font-medium text-sm">{person.name}</p>
                      {person.character && (
                        <p className="text-xs text-muted-foreground">{person.character}</p>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Sidebar */}
          <div className="space-y-6">
            {/* Poster */}
            <div className="aspect-[2/3] relative rounded-lg overflow-hidden">
              {movie.metadata.poster ? (
                <Image
                  src={movie.metadata.poster}
                  alt={movie.title}
                  fill
                  className="object-cover"
                />
              ) : (
                <div className="w-full h-full bg-gray-200 dark:bg-gray-700 flex items-center justify-center">
                  <span className="text-gray-400">No Poster</span>
                </div>
              )}
            </div>

            {/* Details */}
            <div className="space-y-4">
              <h3 className="text-lg font-semibold">Details</h3>
              
              {movie.metadata.releaseDate && (
                <div>
                  <dt className="font-medium">Release Date</dt>
                  <dd className="text-muted-foreground">{formatDate(movie.metadata.releaseDate)}</dd>
                </div>
              )}
              
              {movie.duration && (
                <div>
                  <dt className="font-medium">Runtime</dt>
                  <dd className="text-muted-foreground">{formatDuration(movie.duration)}</dd>
                </div>
              )}
              
              <div>
                <dt className="font-medium">File Size</dt>
                <dd className="text-muted-foreground">
                  {(movie.fileSize / (1024 * 1024 * 1024)).toFixed(2)} GB
                </dd>
              </div>

              {movie.metadata.imdbId && (
                <div>
                  <dt className="font-medium">IMDb</dt>
                  <dd>
                    <a
                      href={`https://www.imdb.com/title/${movie.metadata.imdbId}`}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-blue-600 hover:text-blue-500"
                    >
                      View on IMDb
                    </a>
                  </dd>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Video Player Modal */}
        <Modal
          isOpen={isPlayerOpen}
          onClose={() => setIsPlayerOpen(false)}
          size="xl"
          title={movie.title}
        >
          {streamUrl && (
            <VideoPlayer
              src={streamUrl}
              mediaId={movieId}
              poster={movie.metadata.backdrop || movie.metadata.poster}
              initialTime={playbackState?.position || 0}
            />
          )}
        </Modal>
      </div>
    </Layout>
  )
}
