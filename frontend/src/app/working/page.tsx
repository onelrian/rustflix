'use client'

import { useEffect, useState } from 'react'

export default function WorkingPage() {
  const [apiData, setApiData] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    // Test direct API call to backend
    fetch('http://localhost:8080/api/v1/media?limit=2')
      .then(res => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`)
        return res.json()
      })
      .then(data => {
        setApiData(data)
        setLoading(false)
      })
      .catch(err => {
        setError(err.message)
        setLoading(false)
      })
  }, [])

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-4">
          🎬 RustFlix - Working Integration Test
        </h1>
        <p className="text-gray-600">
          Testing direct frontend ↔ backend integration without complex providers
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold mb-4">Backend API Status</h2>
          
          {loading && (
            <div className="flex items-center space-x-2">
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"></div>
              <span>Testing connection...</span>
            </div>
          )}
          
          {error && (
            <div className="text-red-600">
              ❌ Connection failed: {error}
            </div>
          )}
          
          {apiData && (
            <div className="text-green-600">
              ✅ Backend connected successfully!
              <div className="mt-2 text-sm text-gray-600">
                Found {apiData.data?.length || 0} media items
              </div>
            </div>
          )}
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold mb-4">Frontend Status</h2>
          <div className="space-y-2">
            <div className="text-green-600">✅ Next.js routing working</div>
            <div className="text-green-600">✅ React components rendering</div>
            <div className="text-green-600">✅ Tailwind CSS styling</div>
            <div className="text-green-600">✅ Client-side JavaScript</div>
          </div>
        </div>
      </div>

      {apiData && (
        <div className="mt-8 bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold mb-4">Sample Media Data</h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {apiData.data?.map((item: any, index: number) => (
              <div key={index} className="border rounded-lg p-4">
                <h3 className="font-semibold">{item.title}</h3>
                <p className="text-sm text-gray-600 mt-1">{item.description}</p>
                <div className="mt-2 flex flex-wrap gap-1">
                  {item.metadata?.genres?.map((genre: string, i: number) => (
                    <span key={i} className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">
                      {genre}
                    </span>
                  ))}
                </div>
                <div className="mt-2 text-xs text-gray-500">
                  {item.mediaType} • {item.metadata?.rating}/10
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="mt-8 bg-gray-100 rounded-lg p-6">
        <h2 className="text-xl font-semibold mb-4">Raw API Response</h2>
        <pre className="text-xs overflow-auto bg-white p-4 rounded border">
          {JSON.stringify(apiData, null, 2)}
        </pre>
      </div>
    </div>
  )
}
