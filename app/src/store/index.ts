import { InjectionKey } from 'vue'
import { createStore, Store } from 'vuex'

export interface State {
  posts: Map<string, string>
}

export const key: InjectionKey<Store<State>> = Symbol()

export const store = createStore<State>({
  state: {
    posts: new Map<string, string>()
  },
})
