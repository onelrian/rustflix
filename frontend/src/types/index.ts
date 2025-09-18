export interface User {
  id: string
  username: string
  email: string
  role: 'admin' | 'user'
  preferences: UserPreferences
  createdAt: string
  updatedAt: string
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'system'
  language: string
  autoplay: boolean
  subtitles: boolean
  quality: 'auto' | '480p' | '720p' | '1080p' | '4k'
}

export interface MediaItem {
  id: string
  title: string
  description?: string
  mediaType: 'movie' | 'tv_show' | 'music' | 'other'
  filePath: string
  fileSize: number
  duration?: number
  metadata: MediaMetadata
  createdAt: string
  updatedAt: string
}

export interface MediaMetadata {
  title: string
  description?: string
  releaseDate?: string
  genres: string[]
  cast: Person[]
  crew: Person[]
  rating?: number
  poster?: string
  backdrop?: string
  trailer?: string
  imdbId?: string
  tmdbId?: number
}

export interface Person {
  id: string
  name: string
  role: string
  character?: string
  profileImage?: string
}

export interface TVShow extends MediaItem {
  seasons: Season[]
  totalSeasons: number
  totalEpisodes: number
}

export interface Season {
  id: string
  showId: string
  seasonNumber: number
  title: string
  description?: string
  episodes: Episode[]
  poster?: string
  airDate?: string
}

export interface Episode {
  id: string
  seasonId: string
  episodeNumber: number
  title: string
  description?: string
  duration?: number
  filePath: string
  airDate?: string
  thumbnail?: string
}

export interface StreamingSession {
  id: string
  mediaId: string
  userId: string
  streamUrl: string
  format: 'hls' | 'dash' | 'progressive'
  quality: string
  startTime: string
  endTime?: string
}

export interface PlaybackState {
  mediaId: string
  userId: string
  position: number
  duration: number
  completed: boolean
  lastWatched: string
}

export interface SearchFilters {
  query?: string
  mediaType?: 'movie' | 'tv_show' | 'music' | 'all'
  genres?: string[]
  year?: number
  rating?: number
  sortBy?: 'title' | 'releaseDate' | 'rating' | 'duration'
  sortOrder?: 'asc' | 'desc'
  page?: number
  limit?: number
}

export interface ApiResponse<T> {
  data: T
  success: boolean
  message?: string
}

export interface PaginatedResponse<T> {
  data: T[]
  pagination: {
    page: number
    limit: number
    total: number
    totalPages: number
  }
}

export interface LoginRequest {
  username: string
  password: string
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
}

export interface AuthResponse {
  user: User
  token: string
}

export interface ErrorResponse {
  error: string
  message: string
  statusCode: number
}
