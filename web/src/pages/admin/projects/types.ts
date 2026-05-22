import { Project } from '../../user/projects/types'

export type AdminProject = Project & {
  username: string
}
