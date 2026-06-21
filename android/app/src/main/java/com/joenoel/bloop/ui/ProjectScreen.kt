package com.joenoel.bloop.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GraphicEq
import androidx.compose.material.icons.filled.Repeat
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import bloop.Bloop
import bloop.request
import bloop.selectRequest
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState
import com.joenoel.bloop.ui.theme.BloopNeutral2
import com.joenoel.bloop.ui.theme.BloopTheme1

@Composable
fun ProjectScreen(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
) {
    Surface(
        modifier = Modifier.fillMaxSize(),
        color = MaterialTheme.colorScheme.background,
    ) {
        Scaffold(
            modifier = Modifier.fillMaxSize(),
            containerColor = MaterialTheme.colorScheme.background,
            bottomBar = {
                TransportBar(
                    state = state,
                    onDispatch = onDispatch,
                )
            },
        ) { innerPadding ->
            ProjectContent(
                state = state,
                onDispatch = onDispatch,
                modifier = Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
            )
        }
    }
}

@Composable
private fun ProjectContent(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    modifier: Modifier = Modifier,
) {
    val selectedSong = state.project.songsList.firstOrNull { it.id == state.project.selections.song }
    val selectedSection = selectedSong?.sectionsList?.firstOrNull { it.id == state.project.selections.section }

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

    Column(
        modifier = modifier.padding(horizontal = 20.dp, vertical = 20.dp),
    ) {
        Text(
            text = selectedSong.name.takeIf { it.isNotBlank() } ?: "Untitled song",
            style = MaterialTheme.typography.headlineLarge,
            fontWeight = FontWeight.Bold,
            color = MaterialTheme.colorScheme.onBackground,
        )
        Spacer(modifier = Modifier.height(6.dp))
        Text(
            text = selectedSection?.name?.takeIf { it.isNotBlank() } ?: "No section selected",
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(modifier = Modifier.height(16.dp))

        BoxWithConstraints(
            modifier = Modifier
                .fillMaxWidth()
                .weight(1f),
        ) {
            val isCompact = maxWidth < 600.dp
            val columnCount = if (isCompact) 1 else 2

            LazyVerticalGrid(
                columns = GridCells.Fixed(columnCount),
                verticalArrangement = Arrangement.spacedBy(12.dp),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                items(
                    items = selectedSong.sectionsList,
                    key = { it.id },
                ) { section ->
                    SectionCard(
                        section = section,
                        selections = state.project.selections,
                        playbackState = state.playbackState,
                        progress = state.progress,
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
}

@Composable
private fun SectionCard(
    section: Bloop.Section,
    selections: Bloop.Selections,
    playbackState: Bloop.PlaybackState,
    progress: Bloop.Progress,
    onSelect: () -> Unit,
) {
    val isSelected = selections.section == section.id
    val isPlaying = playbackState.sectionId == section.id
    val borderColor = when {
        isPlaying -> BloopTheme1
        isSelected -> BloopNeutral2
        else -> Color.Transparent
    }

    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onSelect),
        shape = RoundedCornerShape(4.dp),
        color = MaterialTheme.colorScheme.surfaceVariant,
    ) {
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .defaultMinSize(minHeight = 64.dp)
                .height(IntrinsicSize.Min),
            contentAlignment = Alignment.CenterStart,
        ) {
            if (isPlaying) {
                Box(
                    modifier = Modifier
                        .fillMaxHeight()
                        .fillMaxWidth(progress.sectionProgress.coerceIn(0.0, 1.0).toFloat())
                        .background(BloopTheme1.copy(alpha = 0.3f)),
                )
            }

            Box(
                modifier = Modifier
                    .fillMaxHeight()
                    .width(4.dp)
                    .background(borderColor),
            )

            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 14.dp, vertical = 12.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = section.name.takeIf { it.isNotBlank() } ?: "Untitled section",
                    style = MaterialTheme.typography.headlineSmall,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onSurface,
                    modifier = Modifier.weight(1f),
                )

                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    if (section.loop) {
                        Icon(
                            imageVector = Icons.Filled.Repeat,
                            contentDescription = "Loop section",
                            tint = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                    if (section.metronome) {
                        Icon(
                            imageVector = Icons.Filled.GraphicEq,
                            contentDescription = "Metronome enabled",
                            tint = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
        }
    }
}
