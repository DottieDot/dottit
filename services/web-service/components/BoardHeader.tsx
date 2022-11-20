import { Box, Typography } from '@mui/material'
import { Fragment } from 'react'
import Glass from './Glass'

export interface BoardHeaderProps {
  board: string
}

export default function BoardHeader({ board }: BoardHeaderProps) {
  return (
    <Fragment>
      <Box
        sx={
          {
            height:               200,
            backgroundImage:      'url("https://picsum.photos/1000/1000")',
            backgroundRepeat:     'no-repeat',
            backgroundSize:       'cover',
            backgroundPosition:   'center',
            backgroundAttachment: 'fixed' 
          }
        }
      />

      <Glass
        sx={
          {
            p:        2,
            position: 'sticky',
            top:      0,
            zIndex:   1 
          }
        }
      >
        <Typography component="h1" variant="h4">
          {board}
        </Typography>
      </Glass>
    </Fragment>
  )
}
