package com.joenoel.bloop.state

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

typealias Reducer = (AppState, AppAction) -> AppState

fun interface AppMiddleware {
    suspend fun execute(state: AppState, action: AppAction, dispatch: (AppAction) -> Unit)
}

class AppStoreViewModel(
    initialState: AppState = AppState(),
    private val reducer: Reducer = AppReducer::reduce,
    private val middlewares: List<AppMiddleware> = emptyList(),
) : ViewModel() {

    private val mutableState = MutableStateFlow(initialState)
    val state: StateFlow<AppState> = mutableState.asStateFlow()

    fun dispatch(action: AppAction) {
        mutableState.update { current -> reducer(current, action) }

        val nextState = mutableState.value
        middlewares.forEach { middleware ->
            viewModelScope.launch {
                middleware.execute(nextState, action, ::dispatch)
            }
        }
    }
}
