import axios from 'axios'
import type {
  User,
  MediaItem,
  TVShow,
  Episode,
  LoginRequest,
  RegisterRequest,
  AuthResponse,
  ApiResponse,
  PaginatedResponse,
  SearchFilters,
  PlaybackState,
  StreamingSession
} from '@/types'

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080'

// Create axios instance with default config
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Add auth token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('authToken')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('authToken')
      window.location.href = '/auth/login'
    }
    return Promise.reject(error)
  }
)

// Auth API
export const authApi = {
  login: async (credentials: LoginRequest): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/api/auth/login', credentials)
    return response.data
  },

  register: async (userData: RegisterRequest): Promise<AuthResponse> => {
    const response = await api.post<AuthResponse>('/api/auth/register', userData)
    return response.data
  },

  logout: async (): Promise<void> => {
    await api.post('/api/auth/logout')
    localStorage.removeItem('authToken')
  },

  getCurrentUser: async (): Promise<User> => {
    const response = await api.get<User>('/api/auth/me')
    return response.data
  },
}

// Media API
export const mediaApi = {
  getMedia: async (filters?: SearchFilters): Promise<PaginatedResponse<MediaItem>> => {
    const response = await api.get<PaginatedResponse<MediaItem>>('/api/v1/media', { params: filters })
    return response.data
  },

  getMediaById: async (id: string): Promise<MediaItem> => {
    const response = await api.get<ApiResponse<MediaItem>>(`/api/v1/media/${id}`)
    return response.data.data
  },

  searchMedia: async (query: string, filters?: SearchFilters): Promise<PaginatedResponse<MediaItem>> => {
    const response = await api.get<PaginatedResponse<MediaItem>>('/api/v1/media/search', {
      params: { q: query, ...filters }
    })
    return response.data
  },

  getGenres: async (): Promise<string[]> => {
    const response = await api.get<ApiResponse<string[]>>('/api/v1/media/genres')
    return response.data.data
  },

  getTVShow: async (id: string): Promise<TVShow> => {
    const response = await api.get<ApiResponse<TVShow>>(`/api/v1/media/tv/${id}`)
    return response.data.data
  },

  getEpisode: async (showId: string, seasonNumber: number, episodeNumber: number): Promise<Episode> => {
    const response = await api.get<ApiResponse<Episode>>(`/api/v1/media/tv/${showId}/season/${seasonNumber}/episode/${episodeNumber}`)
    return response.data.data
  },

  getLibraries: async (): Promise<any[]> => {
    const response = await api.get<ApiResponse<any[]>>('/api/v1/libraries')
    return response.data.data || []
  },

  scanLibrary: async (id: string): Promise<void> => {
    await api.post(`/api/v1/libraries/${id}/scan`)
  },
}

// User Management API
export const userApi = {
  getUsers: async (): Promise<User[]> => {
    const response = await api.get<ApiResponse<User[]>>('/api/v1/users')
    return response.data.data || []
  },

  getUserById: async (id: string): Promise<User> => {
    const response = await api.get<ApiResponse<User>>(`/api/v1/users/${id}`)
    return response.data.data
  },

  createUser: async (userData: any): Promise<User> => {
    const response = await api.post<ApiResponse<User>>('/api/v1/users', userData)
    return response.data.data
  },

  updateUser: async (id: string, userData: any): Promise<User> => {
    const response = await api.put<ApiResponse<User>>(`/api/v1/users/${id}`, userData)
    return response.data.data
  },

  deleteUser: async (id: string): Promise<void> => {
    await api.delete(`/api/v1/users/${id}`)
  },
}

// Streaming API
export const streamingApi = {
  createSession: async (mediaId: string): Promise<StreamingSession> => {
    const response = await api.post<ApiResponse<StreamingSession>>('/api/v1/stream/session', { mediaId })
    return response.data.data
  },

  updatePlaybackState: async (sessionId: string, state: PlaybackState): Promise<void> => {
    await api.put(`/api/v1/stream/session/${sessionId}/state`, state)
  },

  getPlaybackState: async (sessionId: string): Promise<PlaybackState> => {
    const response = await api.get<ApiResponse<PlaybackState>>(`/api/v1/stream/session/${sessionId}/state`)
    return response.data.data
  },

  endSession: async (sessionId: string): Promise<void> => {
    await api.delete(`/api/v1/stream/session/${sessionId}`)
  },
}

// Admin API
export const adminApi = {
  scanLibrary: async (): Promise<void> => {
    await api.post('/api/v1/libraries/scan')
  },

  getSystemStats: async (): Promise<any> => {
    // Mock data for now since backend doesn't have this endpoint yet
    return {
      totalMedia: 0,
      totalUsers: 0,
      storageUsed: '0 GB',
      activeStreams: 0
    }
  },

  refreshMetadata: async (): Promise<void> => {
    // Mock implementation
    await new Promise(resolve => setTimeout(resolve, 1000))
  },

  cleanDatabase: async (): Promise<void> => {
    // Mock implementation
    await new Promise(resolve => setTimeout(resolve, 1000))
  },

  getServerSettings: async (): Promise<any> => {
    // Mock implementation
    return {
      serverName: 'RustFlix Server',
      version: '1.0.0',
      maxStreams: 10
    }
  },

  updateServerSettings: async (settings: any): Promise<void> => {
    // Mock implementation
    await new Promise(resolve => setTimeout(resolve, 1000))
  },
}

export default api
