import { Container, Typography } from '@mui/material'
import Glass from './Glass'

export interface ThreadHeaderProps {
  thread: string
}

export default function ThreadHeader({ thread }: ThreadHeaderProps) {
  return (
    <Glass
      sx={{
        p:        2,
        position: 'sticky',
        top:      0,
        zIndex:   1
      }}
    >
      <Container maxWidth="md">
        <Typography component="h1" variant="h4">
          {thread}
        </Typography>
      </Container>
    </Glass>
  )
}
