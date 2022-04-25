import HomeView from './views/HomeView.svelte'
import BlogView from './views/BlogView.svelte'
import NotFoundView from './views/NotFoundView.svelte'

const routes = [
  {
    name: "/",
    component: HomeView,
  },
  {
    name: "blog",
    component: BlogView,
  },
  {
    name: "404",
    component: NotFoundView,
  },
]

export { routes };
