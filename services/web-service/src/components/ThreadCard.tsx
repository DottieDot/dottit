import { Box, ButtonBase, IconButton, Typography } from '@mui/material'
import { North as UpvoteIcon, ChatBubbleOutline as CommentsIcon, South as DownvoteIcon } from '@mui/icons-material'
import { memo, useCallback } from 'react'
import UserLink from './UserLink'

export interface ThreadCardProps {
  id: string
  title: string
  username?: string
  text: string | null
  onClick: (threadId: string) => void
}

function ThreadCard({ id, title, username, text, onClick }: ThreadCardProps) {
  const handleClick = useCallback(() => {
    onClick(id)
  }, [ onClick, id ])

  return (
    <Box
      sx={{
        borderBottom: '4px solid #444',
        width:        '100%',
        display:      'block'
      }}
    >
      <ButtonBase
        component={Box}
        onClick={handleClick}
        sx={{
          display: 'block',
          p:       2
        }}
      >
        <Typography variant="h5" gutterBottom>
          {title}
        </Typography>

        <Typography variant="body2">
          {text}
        </Typography>
      </ButtonBase>

      <Box
        sx={{
          display: 'flex',
          p:       2,
          pt:      0
        }}
      >
        <Box sx={{ flex: 1 }}>
          <Box>
            {username && (
              <Typography color="text.secondary" sx={{ fontSize: 14 }}>
                By
                {' '}

                <UserLink>
                  {username}
                </UserLink>
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
              {0}
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
    </Box>
  )
}

export default memo(ThreadCard)
