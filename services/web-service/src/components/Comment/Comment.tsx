import { alpha, Box, Card, CardContent, IconButton, styled, Typography } from '@mui/material'
import { North as UpvoteIcon, South as DownvoteIcon } from '@mui/icons-material'
import { memo } from 'react'
import UserLink from '../UserLink'

const Content = styled(CardContent)({
  display:             'grid',
  gridTemplateColumns: 'auto 1fr',
  padding:             0,
  paddingBottom:       'unset !important'
})

const Score = styled('div')(({ theme }) => ({
  display:        'flex',
  flexDirection:  'column',
  justifyContent: 'flex-start',
  alignItems:     'center',
  background:     alpha(theme.palette.text.primary, .1),
  userSelect:     'none'
}))

export interface CommentProps {
  user: string
  text: string
}

function Comment({ user, text }: CommentProps) {
  return (
    <Card>
      <Content>
        <Score>
          <IconButton size="small">
            <UpvoteIcon fontSize="inherit"  />
          </IconButton>

          <Box component="span" sx={{ cursor: 'default' }}>
            0
          </Box>

          <IconButton size="small">
            <DownvoteIcon fontSize="inherit" />
          </IconButton>
        </Score>

        <Box
          sx={{
            px: 2,
            py: 1
          }}
        >
          <Typography variant="subtitle1" gutterBottom>
            <UserLink>
              {user}
            </UserLink>
          </Typography>

          <Typography sx={{ whiteSpace: 'pre-wrap' }} variant="body2">
            {text}
          </Typography>
        </Box>
      </Content>
    </Card>
  )
}

export default memo(Comment)
