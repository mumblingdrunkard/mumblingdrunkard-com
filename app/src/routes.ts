import HomeView from './views/HomeView.svelte'
import BlogView from './views/BlogView.svelte'
import ProjectsView from './views/ProjectsView.svelte'
import AboutView from './views/AboutView.svelte'
import ContactView from './views/ContactView.svelte'
import Layout from './Layout.svelte'
import NotFoundView from './views/NotFoundView.svelte'

const routes = [
  {
    name: "/",
    component: HomeView,
    layout: Layout,
  },
  {
    name: "blog",
    component: BlogView,
    layout: Layout,
  },
  {
    name: "projects",
    component: ProjectsView,
    layout: Layout,
  },
  {
    name: "about",
    component: AboutView,
    layout: Layout,
  },
  {
    name: "contact",
    component: ContactView,
    layout: Layout,
  },
  {
    name: "404",
    component: NotFoundView,
    layout: Layout,
  },
]

export { routes };
