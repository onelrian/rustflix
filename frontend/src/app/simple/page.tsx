'use client'

import { useEffect, useState } from 'react'

export default function SimplePage() {
  const [mediaData, setMediaData] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    // Test API call to backend
    fetch('http://localhost:8080/api/v1/media?limit=3')
      .then(res => res.json())
      .then(data => {
        setMediaData(data)
        setLoading(false)
      })
      .catch(err => {
        setError(err.message)
        setLoading(false)
      })
  }, [])

  if (loading) {
    return <div className="p-8">Loading...</div>
  }

  if (error) {
    return <div className="p-8 text-red-600">Error: {error}</div>
  }

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-6">RustFlix - Simple Test</h1>
      
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Backend API Test</h2>
        <p className="text-green-600">✅ Successfully connected to backend!</p>
      </div>

      <div>
        <h2 className="text-xl font-semibold mb-4">Media Data from Backend:</h2>
        <pre className="bg-gray-100 p-4 rounded text-sm overflow-auto">
          {JSON.stringify(mediaData, null, 2)}
        </pre>
      </div>

      <div className="mt-8">
        <a href="/test" className="text-blue-600 hover:underline mr-4">Test Page</a>
        <a href="/auth/login" className="text-blue-600 hover:underline">Login Page</a>
      </div>
    </div>
  )
}
