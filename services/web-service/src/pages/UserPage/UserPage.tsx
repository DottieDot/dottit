import { Box, Paper, Typography } from '@mui/material'
import { useParams } from 'react-router'
import { useQuery } from '@apollo/client'
import { getUserByUsernameQuery, GetUserByUsernameResponse } from './api'

function Content() {
  const { user: userParam } = useParams()
  const { data, loading }  = useQuery<GetUserByUsernameResponse>(getUserByUsernameQuery, { variables: { username: userParam }})

  if (loading) {
    return (
      <div>
        <Typography variant="h2">
          Loading
        </Typography>
      </div>
    )
  }
  if (!data?.getUserByUsername) {
    return (
      <div>
        <Typography variant="h2">
          User not found
        </Typography>

        <Typography variant="h4">
          No user with the username &quot;
          {userParam}
          &quot; could be found.
        </Typography>
      </div>
    )
  }

  const user = data.getUserByUsername

  return (
    <Paper sx={{ p: 2 }}>
      <Typography variant="h4">
        {user.username}
      </Typography>
    </Paper>
  )
}

export default function UsePage() {
  return (
    <Box
      sx={{
        display:        'flex',
        alignItems:     'center',
        justifyContent: 'center',
        flex:           1
      }}
    >
      <Content />
    </Box>
  )
}
