import { Link } from '@mui/material'
import { Link as RouterLink } from'react-router-dom'

export interface UserLinkProps {
  children: string
}

export default function UserLink({ children: user }: UserLinkProps) {
  return (
    <Link
      color="inherit"
      component={RouterLink}
      to={`/u/${user}`}
      underline="hover"
    >
      {user}
    </Link>
  )
}
