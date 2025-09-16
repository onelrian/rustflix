/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  images: {
    domains: ['localhost', 'image.tmdb.org', 'images.unsplash.com'],
    unoptimized: true
  },
  experimental: {
    serverComponentsExternalPackages: ['video.js']
  }
}

export default nextConfig
