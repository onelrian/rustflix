'use client'

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { mediaApi, streamingApi, userApi } from '@/lib/api'
import type { SearchFilters, MediaItem, TVShow, Episode, PlaybackState } from '@/types'

export function useMedia(filters?: SearchFilters) {
  return useQuery({
    queryKey: ['media', filters],
    queryFn: () => mediaApi.getMedia(filters),
  })
}

export function useMediaById(id: string) {
  return useQuery({
    queryKey: ['media', id],
    queryFn: () => mediaApi.getMediaById(id),
    enabled: !!id,
  })
}

export function useSearchMedia(query: string, filters?: SearchFilters) {
  return useQuery({
    queryKey: ['media', 'search', query, filters],
    queryFn: () => mediaApi.searchMedia(query, filters),
    enabled: !!query,
  })
}

export function useGenres() {
  return useQuery({
    queryKey: ['genres'],
    queryFn: () => mediaApi.getGenres(),
  })
}

export function useTVShow(id: string) {
  return useQuery({
    queryKey: ['tv', id],
    queryFn: () => mediaApi.getTVShow(id),
    enabled: !!id,
  })
}

export function useEpisode(showId: string, seasonNumber: number, episodeNumber: number) {
  return useQuery({
    queryKey: ['episode', showId, seasonNumber, episodeNumber],
    queryFn: () => mediaApi.getEpisode(showId, seasonNumber, episodeNumber),
    enabled: !!showId && !!seasonNumber && !!episodeNumber,
  })
}

export function useStreamUrl(mediaId: string, format: 'hls' | 'dash' = 'hls') {
  return useQuery({
    queryKey: ['stream', mediaId, format],
    queryFn: () => streamingApi.getStreamUrl(mediaId, format),
    enabled: !!mediaId,
  })
}

export function usePlaybackState(mediaId: string) {
  return useQuery({
    queryKey: ['playback', mediaId],
    queryFn: () => streamingApi.getPlaybackState(mediaId),
    enabled: !!mediaId,
  })
}

export function useUpdatePlaybackState() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: ({ mediaId, position }: { mediaId: string; position: number }) =>
      streamingApi.updatePlaybackState(mediaId, {
        userId: 'current-user', // TODO: Get from auth context
        mediaId,
        position,
        duration: 0,
        completed: false,
        lastWatched: new Date().toISOString()
      }),
    onSuccess: (_, { mediaId }) => {
      queryClient.invalidateQueries({ queryKey: ['playback', mediaId] })
    },
  })
}

export function useWatchHistory() {
  return useQuery({
    queryKey: ['watchHistory'],
    queryFn: () => userApi.getWatchHistory(),
  })
}

export function useWatchlist() {
  return useQuery({
    queryKey: ['watchlist'],
    queryFn: () => userApi.getWatchlist(),
  })
}

export function useAddToWatchlist() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (mediaId: string) => userApi.addToWatchlist(mediaId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['watchlist'] })
    },
  })
}

export function useRemoveFromWatchlist() {
  const queryClient = useQueryClient()
  
  return useMutation({
    mutationFn: (mediaId: string) => userApi.removeFromWatchlist(mediaId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['watchlist'] })
    },
  })
}
