'use client'

import React, { useState } from 'react'
import { Filter, X } from 'lucide-react'
import { SearchFilters as SearchFiltersType } from '@/types'
import { Button } from '@/components/ui/Button'
import { useGenres } from '@/hooks/useMediaQuery'

interface SearchFiltersProps {
  filters: SearchFiltersType
  onFiltersChange: (filters: SearchFiltersType) => void
}

export function SearchFilters({ filters, onFiltersChange }: SearchFiltersProps) {
  const [isOpen, setIsOpen] = useState(false)
  const { data: genres = [] } = useGenres()

  const updateFilter = (key: keyof SearchFiltersType, value: string | string[] | number) => {
    onFiltersChange({ ...filters, [key]: value })
  }

  const clearFilters = () => {
    onFiltersChange({
      query: filters.query,
      mediaType: 'all',
      genres: [],
      sortBy: 'title',
      sortOrder: 'asc'
    })
  }

  const hasActiveFilters = filters.mediaType !== 'all' || 
    filters.genres?.length || 
    filters.year || 
    filters.rating ||
    filters.sortBy !== 'title' ||
    filters.sortOrder !== 'asc'

  return (
    <div className="relative">
      <Button
        variant="outline"
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center space-x-2"
      >
        <Filter className="h-4 w-4" />
        <span>Filters</span>
        {hasActiveFilters && (
          <span className="bg-blue-500 text-white text-xs rounded-full px-2 py-0.5">
            Active
          </span>
        )}
      </Button>

      {isOpen && (
        <div className="absolute top-full left-0 mt-2 w-80 bg-background border rounded-lg shadow-lg p-4 z-10">
          <div className="flex items-center justify-between mb-4">
            <h3 className="font-semibold">Filters</h3>
            <div className="flex items-center space-x-2">
              {hasActiveFilters && (
                <Button variant="ghost" size="sm" onClick={clearFilters}>
                  Clear
                </Button>
              )}
              <Button variant="ghost" size="sm" onClick={() => setIsOpen(false)}>
                <X className="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div className="space-y-4">
            {/* Media Type */}
            <div>
              <label className="text-sm font-medium mb-2 block">Media Type</label>
              <select
                value={filters.mediaType || 'all'}
                onChange={(e) => updateFilter('mediaType', e.target.value)}
                className="w-full p-2 border rounded-md bg-background"
              >
                <option value="all">All</option>
                <option value="movie">Movies</option>
                <option value="tv_show">TV Shows</option>
                <option value="music">Music</option>
              </select>
            </div>

            {/* Genres */}
            <div>
              <label className="text-sm font-medium mb-2 block">Genres</label>
              <div className="max-h-32 overflow-y-auto space-y-1">
                {genres.map((genre) => (
                  <label key={genre} className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      checked={filters.genres?.includes(genre) || false}
                      onChange={(e) => {
                        const currentGenres = filters.genres || []
                        if (e.target.checked) {
                          updateFilter('genres', [...currentGenres, genre])
                        } else {
                          updateFilter('genres', currentGenres.filter(g => g !== genre))
                        }
                      }}
                      className="rounded"
                    />
                    <span className="text-sm">{genre}</span>
                  </label>
                ))}
              </div>
            </div>

            {/* Year */}
            <div>
              <label className="text-sm font-medium mb-2 block">Year</label>
              <input
                type="number"
                min="1900"
                max={new Date().getFullYear()}
                value={filters.year || ''}
                onChange={(e) => updateFilter('year', e.target.value ? parseInt(e.target.value) : 0)}
                className="w-full p-2 border rounded-md bg-background"
                placeholder="Any year"
              />
            </div>

            {/* Rating */}
            <div>
              <label className="text-sm font-medium mb-2 block">Minimum Rating</label>
              <input
                type="number"
                min="0"
                max="10"
                step="0.1"
                value={filters.rating || ''}
                onChange={(e) => updateFilter('rating', e.target.value ? parseFloat(e.target.value) : 0)}
                className="w-full p-2 border rounded-md bg-background"
                placeholder="Any rating"
              />
            </div>

            {/* Sort */}
            <div className="grid grid-cols-2 gap-2">
              <div>
                <label className="text-sm font-medium mb-2 block">Sort By</label>
                <select
                  value={filters.sortBy || 'title'}
                  onChange={(e) => updateFilter('sortBy', e.target.value)}
                  className="w-full p-2 border rounded-md bg-background"
                >
                  <option value="title">Title</option>
                  <option value="releaseDate">Release Date</option>
                  <option value="rating">Rating</option>
                  <option value="duration">Duration</option>
                </select>
              </div>
              <div>
                <label className="text-sm font-medium mb-2 block">Order</label>
                <select
                  value={filters.sortOrder || 'asc'}
                  onChange={(e) => updateFilter('sortOrder', e.target.value)}
                  className="w-full p-2 border rounded-md bg-background"
                >
                  <option value="asc">Ascending</option>
                  <option value="desc">Descending</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
