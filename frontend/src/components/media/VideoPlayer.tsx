'use client'

import React, { useEffect, useRef, useState } from 'react'
import videojs from 'video.js'
import 'video.js/dist/video-js.css'
import '@videojs/http-streaming'
import { useUpdatePlaybackState } from '@/hooks/useMediaQuery'

interface VideoPlayerProps {
  src: string
  mediaId: string
  poster?: string
  initialTime?: number
  onTimeUpdate?: (currentTime: number) => void
  onEnded?: () => void
}

export function VideoPlayer({
  src,
  mediaId,
  poster,
  initialTime = 0,
  onTimeUpdate,
  onEnded
}: VideoPlayerProps) {
  const videoRef = useRef<HTMLDivElement>(null)
  const playerRef = useRef<any>(null)
  const updatePlaybackState = useUpdatePlaybackState()
  const [lastUpdateTime, setLastUpdateTime] = useState(0)

  useEffect(() => {
    if (!videoRef.current) return

    const videoElement = document.createElement('video-js')
    videoElement.classList.add('vjs-big-play-centered')
    videoRef.current.appendChild(videoElement)

    const player = videojs(videoElement, {
      controls: true,
      responsive: true,
      fluid: true,
      playbackRates: [0.5, 1, 1.25, 1.5, 2],
      poster: poster,
      sources: [{
        src: src,
        type: src.includes('.m3u8') ? 'application/x-mpegURL' : 'video/mp4'
      }],
      html5: {
        vhs: {
          overrideNative: true
        }
      }
    })

    playerRef.current = player

    // Set initial time
    if (initialTime > 0) {
      player.ready(() => {
        player.currentTime(initialTime)
      })
    }

    // Handle time updates
    player.on('timeupdate', () => {
      const currentTime = player.currentTime()
      if (typeof currentTime === 'number') {
        onTimeUpdate?.(currentTime)

        // Update playback state every 10 seconds
        if (currentTime - lastUpdateTime >= 10) {
          updatePlaybackState.mutate({
            mediaId,
            position: currentTime
          })
          setLastUpdateTime(currentTime)
        }
      }
    })

    // Handle video end
    player.on('ended', () => {
      const duration = player.duration()
      if (typeof duration === 'number') {
        updatePlaybackState.mutate({
          mediaId,
          position: duration
        })
      }
      onEnded?.()
    })

    // Handle errors
    player.on('error', (error: any) => {
      console.error('Video player error:', error)
    })

    return () => {
      if (playerRef.current && !playerRef.current.isDisposed()) {
        playerRef.current.dispose()
        playerRef.current = null
      }
    }
  }, [src, mediaId, poster, initialTime, onTimeUpdate, onEnded, updatePlaybackState, lastUpdateTime])

  // Update source when it changes
  useEffect(() => {
    if (playerRef.current && !playerRef.current.isDisposed()) {
      playerRef.current.src({
        src: src,
        type: src.includes('.m3u8') ? 'application/x-mpegURL' : 'video/mp4'
      })
    }
  }, [src])

  return (
    <div className="w-full">
      <div ref={videoRef} className="video-js-container" />
    </div>
  )
}
