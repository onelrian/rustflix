'use client'

import React, { useState, useEffect, Suspense } from 'react'
import { useSearchParams } from 'next/navigation'
import { Layout } from '@/components/layout/Layout'
import { MediaGrid } from '@/components/media/MediaGrid'
import { SearchFilters } from '@/components/media/SearchFilters'
import { useSearchMedia, useWatchlist } from '@/hooks/useMediaQuery'
import { SearchFilters as SearchFiltersType } from '@/types'
import { Search } from 'lucide-react'

function SearchPageContent() {
  const searchParams = useSearchParams()
  const initialQuery = searchParams.get('q') || ''
  
  const [query, setQuery] = useState(initialQuery)
  const [filters, setFilters] = useState<SearchFiltersType>({
    mediaType: 'all',
    sortBy: 'title',
    sortOrder: 'asc'
  })
  
  const { data: searchResults, isLoading } = useSearchMedia(query, filters)
  const { data: watchlistData } = useWatchlist()

  const watchlistIds = watchlistData?.data.map((item: { id: string }) => item.id) || []

  useEffect(() => {
    setQuery(initialQuery)
  }, [initialQuery])

  return (
    <Layout>
      <div className="space-y-6">
        <div className="space-y-4">
          <h1 className="text-3xl font-bold">Search</h1>
          
          <div className="flex flex-col md:flex-row gap-4">
            <div className="flex-1 relative">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <input
                type="search"
                placeholder="Search movies, TV shows, music..."
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                className="h-10 w-full rounded-md border border-input bg-background pl-10 pr-3 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
              />
            </div>
            <SearchFilters filters={filters} onFiltersChange={setFilters} />
          </div>
        </div>

        {query && (
          <div>
            <p className="text-muted-foreground mb-4">
              {searchResults?.pagination.total || 0} results for &quot;{query}&quot;
            </p>
            
            <MediaGrid
              media={searchResults?.data || []}
              watchlistIds={watchlistIds}
              loading={isLoading}
            />
          </div>
        )}

        {!query && (
          <div className="text-center py-12">
            <Search className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
            <h2 className="text-xl font-semibold mb-2">Search for content</h2>
            <p className="text-muted-foreground">
              Enter a search term to find movies, TV shows, and music
            </p>
          </div>
        )}
      </div>
    </Layout>
  )
}

export default function SearchPage() {
  return (
    <Suspense fallback={
      <Layout>
        <div className="space-y-6">
          <div className="space-y-4">
            <h1 className="text-3xl font-bold">Search</h1>
            <div className="animate-pulse">
              <div className="h-10 bg-gray-200 rounded-md mb-4"></div>
              <div className="h-32 bg-gray-200 rounded-md"></div>
            </div>
          </div>
        </div>
      </Layout>
    }>
      <SearchPageContent />
    </Suspense>
  )
}
