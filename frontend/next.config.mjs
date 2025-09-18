/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  images: {
    domains: ['localhost', '127.0.0.1'],
  },
  serverExternalPackages: ['video.js'],
}

export default nextConfig
