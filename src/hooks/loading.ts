import { ref } from 'vue'
export default function useLoading(init = false) {
  const loading = ref(init)
  const toggleLoading = () => (loading.value = !loading.value)
  const setLoading = (val: boolean) => (loading.value = val)
  return {
    loading,
    setLoading,
    toggleLoading
  }
}
