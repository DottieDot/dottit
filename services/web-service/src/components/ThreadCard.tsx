import { Box, ButtonBase, IconButton, Typography } from '@mui/material'
import { North as UpvoteIcon, ChatBubbleOutline as CommentsIcon, South as DownvoteIcon } from '@mui/icons-material'
import { memo, useCallback } from 'react'

export interface ThreadCardThread {
  id: string
  title: string
  user?: string
  board?: string
  text?: string | null
  media?: string | null
  score: number
}

export interface ThreadCardProps {
  thread: ThreadCardThread,
  onClick: (threadId: string) => void
}

function ThreadCard({ thread, onClick }: ThreadCardProps) {
  const handleClick = useCallback(() => {
    onClick(thread.id)
  }, [ onClick, thread.id ])

  return (
    <ButtonBase
      component={Box}
      onClick={handleClick}
      sx={{
        p:            2,
        borderBottom: '4px solid #444',
        width:        '100%',
        display:      'block'
      }}
    >
      <Typography variant="h5" gutterBottom>
        {thread.title}
      </Typography>

      <Typography sx={{ mb: 2 }} variant="body2">
        {thread.text}
      </Typography>

      <Box sx={{ display: 'flex' }}>
        <Box sx={{ flex: 1 }}>
          <Box>
            {thread.user && (
              <Typography color="text.secondary" sx={{ fontSize: 14 }}>
                By
                {' '}
                {thread.user}
              </Typography>
            )}
          </Box>

          <Box
            sx={{
              display:       'flex',
              flexDirection: 'row',
              gap:           1
            }}
          >
            <Typography
              color="text.secondary"
              sx={{
                fontSize:   14,
                alignItems: 'center',
                display:    'inline-flex'
              }}
            >
              {thread.score}
              <UpvoteIcon fontSize="inherit" sx={{ ml: .2 }} />
            </Typography>

            <Typography
              color="text.secondary"
              sx={{
                fontSize:   14,
                alignItems: 'center',
                display:    'inline-flex'
              }}
            >
              0
              {' '}
              <CommentsIcon fontSize="inherit" sx={{ ml: .2 }} />
            </Typography>
          </Box>
        </Box>

        <Box
          sx={{
            display:    'flex',
            alignItems: 'center'
          }}
        >
          <IconButton size="small">
            <UpvoteIcon fontSize="small" />
          </IconButton>

          <IconButton size="small">
            <DownvoteIcon fontSize="small" />
          </IconButton>
        </Box>
      </Box>
    </ButtonBase>
  )
}

export default memo(ThreadCard)
