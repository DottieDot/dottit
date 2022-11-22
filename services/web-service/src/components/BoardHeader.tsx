import { Typography } from '@mui/material'
import Glass from './Glass'

export interface BoardHeaderProps {
  board: string
}

export default function BoardHeader({ board }: BoardHeaderProps) {
  return (
    <Glass
      sx={{
        p:        2,
        position: 'sticky',
        top:      0,
        zIndex:   1
      }}
    >
      <Typography component="h1" variant="h4">
        {board}
      </Typography>
    </Glass>
  )
}
