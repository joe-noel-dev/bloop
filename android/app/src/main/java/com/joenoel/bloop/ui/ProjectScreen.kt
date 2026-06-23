package com.joenoel.bloop.ui

import androidx.compose.foundation.gestures.detectHorizontalDragGestures
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.lazy.grid.rememberLazyGridState
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.QueueMusic
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.Computer
import androidx.compose.material.icons.filled.Folder
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material.icons.outlined.PersonOutline
import androidx.compose.material.icons.outlined.Wifi
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import bloop.Bloop
import bloop.logoutRequest
import bloop.request
import bloop.selectRequest
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProjectScreen(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
) {
    var showSongsSheet by remember { mutableStateOf(false) }
    var showProjectsSheet by remember { mutableStateOf(false) }
    var showServerSelectionSheet by remember { mutableStateOf(false) }
    var showPreferencesSheet by remember { mutableStateOf(false) }
    var showConnectionMenu by remember { mutableStateOf(false) }
    var showAccountMenu by remember { mutableStateOf(false) }
    var showLoginSheet by remember { mutableStateOf(false) }

    val selectedSong = state.project.songsList.firstOrNull { it.id == state.project.selections.song }
    val scrollBehavior = TopAppBarDefaults.pinnedScrollBehavior()

    Surface(
        modifier = Modifier.fillMaxSize(),
        color = MaterialTheme.colorScheme.background,
    ) {
        Scaffold(
            modifier = Modifier
                .fillMaxSize()
                .nestedScroll(scrollBehavior.nestedScrollConnection),
            containerColor = MaterialTheme.colorScheme.background,
            topBar = {
                TopAppBar(
                    title = {
                        Text(
                            text = selectedSong?.name?.takeIf { it.isNotBlank() } ?: "Bloop",
                            fontWeight = FontWeight.SemiBold,
                        )
                    },
                    navigationIcon = {
                        IconButton(onClick = { showSongsSheet = true }) {
                            Icon(
                                imageVector = Icons.AutoMirrored.Filled.QueueMusic,
                                contentDescription = "Songs",
                            )
                        }
                    },
                    actions = {
                        Box {
                            IconButton(onClick = { showAccountMenu = true }) {
                                Icon(
                                    imageVector = if (state.user != null) Icons.Filled.Person else Icons.Outlined.PersonOutline,
                                    contentDescription = "Account",
                                )
                            }
                            DropdownMenu(
                                expanded = showAccountMenu,
                                onDismissRequest = { showAccountMenu = false },
                            ) {
                                if (state.user != null) {
                                    DropdownMenuItem(
                                        text = { Text(state.user.name) },
                                        enabled = false,
                                        onClick = {},
                                    )
                                    HorizontalDivider()
                                    DropdownMenuItem(
                                        text = {
                                            Text(
                                                "Sign Out",
                                                color = MaterialTheme.colorScheme.error,
                                            )
                                        },
                                        onClick = {
                                            showAccountMenu = false
                                            onDispatch(
                                                AppAction.SendRequest(
                                                    request { logout = logoutRequest {} }
                                                )
                                            )
                                        },
                                    )
                                } else {
                                    DropdownMenuItem(
                                        text = { Text("Sign In") },
                                        onClick = {
                                            showAccountMenu = false
                                            showLoginSheet = true
                                        },
                                    )
                                }
                            }
                        }
                        Box {
                            IconButton(onClick = { showConnectionMenu = true }) {
                                Icon(
                                    imageVector = Icons.Filled.MoreVert,
                                    contentDescription = "More options",
                                )
                            }
                            DropdownMenu(
                                expanded = showConnectionMenu,
                                onDismissRequest = { showConnectionMenu = false },
                            ) {
                                DropdownMenuItem(
                                    text = { Text("Projects") },
                                    leadingIcon = {
                                        Icon(Icons.Filled.Folder, contentDescription = null)
                                    },
                                    onClick = {
                                        showConnectionMenu = false
                                        showProjectsSheet = true
                                    },
                                )
                                DropdownMenuItem(
                                    text = { Text("Settings") },
                                    leadingIcon = {
                                        Icon(Icons.Filled.Settings, contentDescription = null)
                                    },
                                    onClick = {
                                        showConnectionMenu = false
                                        showPreferencesSheet = true
                                    },
                                )
                                DropdownMenuItem(
                                    text = { Text("Connect to Server") },
                                    leadingIcon = {
                                        Icon(Icons.Outlined.Wifi, contentDescription = null)
                                    },
                                    onClick = {
                                        showConnectionMenu = false
                                        showServerSelectionSheet = true
                                    },
                                )
                                DropdownMenuItem(
                                    text = { Text("Connect Local") },
                                    leadingIcon = {
                                        Icon(Icons.Filled.Computer, contentDescription = null)
                                    },
                                    onClick = {
                                        showConnectionMenu = false
                                        onDispatch(AppAction.Disconnect)
                                        onDispatch(AppAction.ConnectLocal)
                                    },
                                )
                                HorizontalDivider()
                                DropdownMenuItem(
                                    text = {
                                        Text(
                                            "Disconnect",
                                            color = MaterialTheme.colorScheme.error,
                                        )
                                    },
                                    leadingIcon = {
                                        Icon(
                                            Icons.Filled.Close,
                                            contentDescription = null,
                                            tint = MaterialTheme.colorScheme.error,
                                        )
                                    },
                                    onClick = {
                                        showConnectionMenu = false
                                        onDispatch(AppAction.Disconnect)
                                    },
                                )
                            }
                        }
                    },
                    scrollBehavior = scrollBehavior,
                )
            },
            bottomBar = {
                TransportBar(
                    state = state,
                    onDispatch = onDispatch,
                )
            },
        ) { innerPadding ->
            SongView(
                state = state,
                onDispatch = onDispatch,
                modifier = Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
            )
        }
    }

    if (showSongsSheet) {
        SongsSheet(
            state = state,
            onDispatch = onDispatch,
            onDismiss = { showSongsSheet = false },
        )
    }

    if (showProjectsSheet) {
        ProjectsSheet(
            state = state,
            onDispatch = onDispatch,
            onDismiss = { showProjectsSheet = false },
        )
    }

    if (showPreferencesSheet) {
        ModalBottomSheet(
            onDismissRequest = { showPreferencesSheet = false },
            windowInsets = WindowInsets(0),
        ) {
            PreferencesScreen(
                state = state,
                onDispatch = onDispatch,
                onDismiss = { showPreferencesSheet = false },
            )
        }
    }

    if (showServerSelectionSheet) {
        ModalBottomSheet(
            onDismissRequest = { showServerSelectionSheet = false },
        ) {
            ServerSelectionScreen(
                servers = state.servers,
                scanning = state.scanning,
                onLocalSelected = {
                    showServerSelectionSheet = false
                    onDispatch(AppAction.Disconnect)
                    onDispatch(AppAction.ConnectLocal)
                },
                onServerSelected = { endpoint ->
                    showServerSelectionSheet = false
                    onDispatch(AppAction.Disconnect)
                    onDispatch(AppAction.Connect(endpoint))
                },
                onRestartScan = { onDispatch(AppAction.RestartScan) },
                onCancel = { showServerSelectionSheet = false },
            )
        }
    }

    if (showLoginSheet) {
        ModalBottomSheet(
            onDismissRequest = { showLoginSheet = false },
        ) {
            LoginScreen(
                state = state,
                onDispatch = onDispatch,
                onDismiss = { showLoginSheet = false },
            )
        }
    }
}

@Composable
private fun SongView(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    modifier: Modifier = Modifier,
) {
    val selectedSong = state.project.songsList.firstOrNull { it.id == state.project.selections.song }

    if (selectedSong == null) {
        Box(
            modifier = modifier.padding(horizontal = 24.dp, vertical = 32.dp),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = "No song selected",
                style = MaterialTheme.typography.headlineSmall,
                color = MaterialTheme.colorScheme.onBackground,
                textAlign = TextAlign.Center,
            )
        }
        return
    }

    val gridState = key(selectedSong.id) { rememberLazyGridState() }
    val playingSectionId = state.playbackState.sectionId

    LaunchedEffect(playingSectionId, selectedSong.id) {
        val index = selectedSong.sectionsList.indexOfFirst { it.id == playingSectionId }
        if (index >= 0) {
            gridState.animateScrollToItem(index)
        }
    }

    BoxWithConstraints(modifier = modifier) {
        val isCompact = maxWidth < 600.dp
        val columnCount = if (isCompact) 1 else 2

        LazyVerticalGrid(
            state = gridState,
            columns = GridCells.Fixed(columnCount),
            verticalArrangement = Arrangement.spacedBy(12.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            modifier = Modifier
                .fillMaxSize()
                .padding(horizontal = 20.dp, vertical = 20.dp)
                .pointerInput(selectedSong.id, state.project.songsList.size) {
                    var swipeAccumulator = 0f
                    detectHorizontalDragGestures(
                        onDragEnd = {
                            val offset = when {
                                swipeAccumulator < -60f -> 1
                                swipeAccumulator > 60f -> -1
                                else -> 0
                            }
                            if (offset != 0) {
                                selectSongWithOffset(state.project, offset, onDispatch)
                            }
                            swipeAccumulator = 0f
                        },
                        onHorizontalDrag = { _, amount ->
                            swipeAccumulator += amount
                        },
                    )
                },
        ) {
            items(
                items = selectedSong.sectionsList,
                key = { it.id },
            ) { section ->
                SectionCard(
                    section = section,
                    selections = state.project.selections,
                    playbackState = state.playbackState,
                    progress = if (state.playbackState.sectionId == section.id) state.progress else Bloop.Progress.getDefaultInstance(),
                    onSelect = {
                        if (state.project.selections.section != section.id) {
                            onDispatch(
                                AppAction.SendRequest(
                                    request {
                                        select = selectRequest {
                                            entity = Bloop.Entity.SECTION
                                            id = section.id
                                        }
                                    }
                                )
                            )
                        }
                    },
                )
            }
        }
    }
}

private fun selectSongWithOffset(
    project: Bloop.Project,
    offset: Int,
    onDispatch: (AppAction) -> Unit,
) {
    val selectedSongIndex = project.songsList.indexOfFirst { it.id == project.selections.song }
    if (selectedSongIndex == -1) return

    val nextIndex = selectedSongIndex + offset
    if (nextIndex !in project.songsList.indices) return

    val nextSongId = project.songsList[nextIndex].id
    onDispatch(
        AppAction.SendRequest(
            request {
                select = selectRequest {
                    entity = Bloop.Entity.SONG
                    id = nextSongId
                }
            }
        )
    )
}
