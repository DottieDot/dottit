import { memo } from 'react'
import { Card, CardActionArea, CardContent, Typography } from '@mui/material'
import { Link as RouterLink } from 'react-router-dom'

export interface BoardProps {
  name: string
}

function Board({ name }: BoardProps) {
  return (
    <Card>
      <CardActionArea component={RouterLink} to={`/b/${name}`}>
        <CardContent>
          <Typography variant="h6">
            b/
            {name}
          </Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  )
}

export default memo(Board)
