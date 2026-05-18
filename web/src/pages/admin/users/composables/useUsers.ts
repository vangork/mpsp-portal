import { Ref, ref, watch, computed } from 'vue'
import type { Filters, Pagination } from '../../../../data/pages/users'
import { useUsersStore } from '../../../../stores/users'
import { User } from '../types'

const makePaginationRef = () => ref<Pagination>({ page: 1, perPage: 10, total: 0 })
const makeFiltersRef = () => ref<Partial<Filters>>({ isActive: true, search: '' })

export const useUsers = (options?: { pagination?: Ref<Pagination>; filters?: Ref<Partial<Filters>> }) => {
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const usersStore = useUsersStore()

  const { filters = makeFiltersRef(), pagination = makePaginationRef() } = options || {}

  const fetch = async () => {
    isLoading.value = true
    try {
      await usersStore.getAll()
      pagination.value.total = usersStore.items.length
    } finally {
      isLoading.value = false
    }
  }

  watch(
    filters,
    () => {
      // Reset pagination to first page when filters changed
      pagination.value.page = 1
    },
    { deep: true },
  )

  fetch()

  const filteredUsers = computed(() => {
    const users = usersStore.items.filter((u) => {
      if (filters.value.isActive !== undefined && u.active !== filters.value.isActive) {
        return false
      }
      if (filters.value.search) {
        const search = filters.value.search.toLowerCase()
        return u.username.toLowerCase().startsWith(search) || u.email.toLowerCase().startsWith(search)
      }
      return true
    })
    return users
  })

  const users = computed(() => {
    const paginated = filteredUsers.value.slice(
      (pagination.value.page - 1) * pagination.value.perPage,
      pagination.value.page * pagination.value.perPage,
    )
    return paginated
  })

  watch(
    () => filteredUsers.value.length,
    (val) => {
      pagination.value.total = val
    },
    { immediate: true },
  )

  watch(
    [() => filters.value, () => pagination.value.perPage],
    () => {
      pagination.value.page = 1
    },
    { deep: true },
  )

  return {
    error,
    isLoading,
    filters,
    pagination,
    users,

    fetch,

    async add(user: User) {
      isLoading.value = true
      try {
        error.value = null
        return await usersStore.add(user)
      } catch (e) {
        error.value = e as string
      } finally {
        isLoading.value = false
      }
    },

    async update(user: User) {
      isLoading.value = true
      try {
        error.value = null
        return await usersStore.update(user)
      } catch (e) {
        error.value = e as string
      } finally {
        isLoading.value = false
      }
    },
  }
}
