'use client'

import React, { useState } from 'react'
import { Layout } from '@/components/layout/Layout'
import { MediaGrid } from '@/components/media/MediaGrid'
import { SearchFilters } from '@/components/media/SearchFilters'
import { useMedia, useWatchlist } from '@/hooks/useMediaQuery'
import { SearchFilters as SearchFiltersType } from '@/types'
import { Button } from '@/components/ui/Button'
import { ChevronLeft, ChevronRight } from 'lucide-react'

export default function MoviesPage() {
  const [filters, setFilters] = useState<SearchFiltersType>({
    mediaType: 'movie',
    sortBy: 'title',
    sortOrder: 'asc'
  })
  const [page, setPage] = useState(1)
  
  const { data: moviesData, isLoading } = useMedia({ ...filters, page, limit: 24 })
  const { data: watchlistData } = useWatchlist()

  const watchlistIds = watchlistData?.data.map(item => item.id) || []

  const handleFiltersChange = (newFilters: SearchFiltersType) => {
    setFilters(newFilters)
    setPage(1) // Reset to first page when filters change
  }

  const totalPages = moviesData?.pagination.totalPages || 1

  return (
    <Layout>
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <h1 className="text-3xl font-bold">Movies</h1>
          <SearchFilters filters={filters} onFiltersChange={handleFiltersChange} />
        </div>

        <MediaGrid
          media={moviesData?.data || []}
          watchlistIds={watchlistIds}
          loading={isLoading}
        />

        {/* Pagination */}
        {totalPages > 1 && (
          <div className="flex items-center justify-center space-x-4">
            <Button
              variant="outline"
              onClick={() => setPage(p => Math.max(1, p - 1))}
              disabled={page === 1}
            >
              <ChevronLeft className="h-4 w-4 mr-1" />
              Previous
            </Button>
            
            <span className="text-sm text-muted-foreground">
              Page {page} of {totalPages}
            </span>
            
            <Button
              variant="outline"
              onClick={() => setPage(p => Math.min(totalPages, p + 1))}
              disabled={page === totalPages}
            >
              Next
              <ChevronRight className="h-4 w-4 ml-1" />
            </Button>
          </div>
        )}
      </div>
    </Layout>
  )
}
