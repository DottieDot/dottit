import { memo, useEffect, useMemo, useRef } from 'react'
import { Box, SxProps } from '@mui/material'

export interface VisibleTriggerProps {
  onVisibilityChange: (visible: boolean) => void,
  sx?: SxProps
}

function VisibilityTrigger({ onVisibilityChange, sx }: VisibleTriggerProps) {
  const container = useRef<HTMLDivElement>(null)
  const observer = useMemo(() => {
    return new IntersectionObserver(([ entry ]) => {
      onVisibilityChange(entry.isIntersecting)
    })
  }, [ onVisibilityChange ])

  useEffect(() => {
    if (container.current) {
      const element = container.current
      observer.observe(element)

      return () => {
        observer.unobserve(element)
      }
    }
  }, [ observer, container, onVisibilityChange ])

  return <Box ref={container} sx={sx} />
}

export default memo(VisibilityTrigger)
