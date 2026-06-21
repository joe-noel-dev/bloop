package com.joenoel.bloop.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material.icons.filled.Repeat
import androidx.compose.material.icons.filled.SkipNext
import androidx.compose.material.icons.filled.SkipPrevious
import androidx.compose.material.icons.filled.Stop
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import bloop.Bloop
import bloop.queueRequest
import bloop.request
import bloop.selectRequest
import bloop.transportRequest
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState
import kotlin.math.floor

@Composable
fun TransportBar(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    modifier: Modifier = Modifier,
) {
    val playbackState = state.playbackState
    val project = state.project
    val isPlaying = playbackState.playing == Bloop.PlayingState.PLAYING

    val selectedSong = project.songsList.firstOrNull { it.id == project.selections.song }
    val selectedSection = selectedSong?.sectionsList?.firstOrNull { it.id == project.selections.section }
    val playingSong = project.songsList.firstOrNull { it.id == playbackState.songId }
    val playingSection = playingSong?.sectionsList?.firstOrNull { it.id == playbackState.sectionId }

    val displaySong = if (isPlaying) playingSong ?: selectedSong else selectedSong
    val displaySection = if (isPlaying) playingSection ?: selectedSection else selectedSection
    val selectedSongIndex = project.songsList.indexOfFirst { it.id == project.selections.song }
    val hasPreviousSong = selectedSongIndex > 0
    val hasNextSong = selectedSongIndex >= 0 && selectedSongIndex < project.songsList.lastIndex
    val queueState = queueState(state)

    Surface(
        modifier = modifier
            .fillMaxWidth()
            .navigationBarsPadding(),
        color = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.96f),
        shape = RoundedCornerShape(topStart = 20.dp, topEnd = 20.dp),
        tonalElevation = 2.dp,
        shadowElevation = 18.dp,
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 20.dp, vertical = 12.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Box(
                modifier = Modifier
                    .size(width = 44.dp, height = 4.dp)
                    .background(
                        MaterialTheme.colorScheme.onSurface.copy(alpha = 0.12f),
                        RoundedCornerShape(999.dp),
                    )
            )
            Spacer(modifier = Modifier.height(12.dp))
            MetronomeIndicator(
                isPlaying = isPlaying,
                sectionBeat = state.progress.sectionBeat,
            )
            Spacer(modifier = Modifier.height(8.dp))
            BeatPositionRow(
                isPlaying = isPlaying,
                sectionBeat = state.progress.sectionBeat,
                sectionStart = playingSection?.start ?: 0.0,
            )
            Spacer(modifier = Modifier.height(10.dp))
            Text(
                text = displaySong?.name?.takeIf { it.isNotBlank() } ?: "No song selected",
                style = MaterialTheme.typography.headlineSmall,
                fontWeight = FontWeight.SemiBold,
                color = MaterialTheme.colorScheme.onSurface,
                textAlign = TextAlign.Center,
            )
            Spacer(modifier = Modifier.height(4.dp))
            Text(
                text = displaySection?.name?.takeIf { it.isNotBlank() } ?: "No section selected",
                style = MaterialTheme.typography.bodyLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
                textAlign = TextAlign.Center,
            )
            Spacer(modifier = Modifier.height(10.dp))
            Box(modifier = Modifier.fillMaxWidth()) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    TransportIconButton(
                        onClick = { selectSongWithOffset(project, -1, onDispatch) },
                        enabled = hasPreviousSong,
                        iconSize = 26.dp,
                        icon = { modifier ->
                            Icon(Icons.Filled.SkipPrevious, contentDescription = "Previous song", modifier = modifier)
                        },
                    )
                    TransportIconButton(
                        onClick = { selectSongWithOffset(project, 1, onDispatch) },
                        enabled = hasNextSong,
                        iconSize = 26.dp,
                        icon = { modifier ->
                            Icon(Icons.Filled.SkipNext, contentDescription = "Next song", modifier = modifier)
                        },
                    )
                }
                Row(
                    modifier = Modifier.align(Alignment.Center),
                    horizontalArrangement = Arrangement.spacedBy(28.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    TransportIconButton(
                        onClick = {
                            onDispatch(
                                AppAction.SendRequest(
                                    request {
                                        transport = transportRequest {
                                            method = if (playbackState.looping) {
                                                Bloop.TransportMethod.EXIT_LOOP
                                            } else {
                                                Bloop.TransportMethod.LOOP
                                            }
                                        }
                                    }
                                )
                            )
                        },
                        enabled = isPlaying,
                        highlighted = playbackState.looping,
                        iconSize = 30.dp,
                        icon = { modifier ->
                            Icon(Icons.Filled.Repeat, contentDescription = "Toggle loop", modifier = modifier)
                        },
                    )
                    TransportIconButton(
                        onClick = {
                            onDispatch(
                                AppAction.SendRequest(
                                    request {
                                        transport = transportRequest {
                                            method = if (isPlaying) {
                                                Bloop.TransportMethod.STOP
                                            } else {
                                                Bloop.TransportMethod.PLAY
                                            }
                                        }
                                    }
                                )
                            )
                        },
                        emphasized = true,
                        iconSize = 34.dp,
                        icon = { modifier ->
                            Icon(
                                imageVector = if (isPlaying) Icons.Filled.Stop else Icons.Filled.PlayArrow,
                                contentDescription = if (isPlaying) "Stop" else "Play",
                                modifier = modifier,
                            )
                        },
                    )
                    when (queueState) {
                        QueueState.QUEUED -> {
                            TransportIconButton(
                                onClick = {},
                                enabled = false,
                                highlighted = true,
                                iconSize = 28.dp,
                                icon = { modifier ->
                                    Icon(Icons.Filled.Check, contentDescription = "Queued", modifier = modifier)
                                },
                            )
                        }
                        QueueState.READY -> {
                            TransportIconButton(
                                onClick = {
                                    val songId = project.selections.song
                                    val sectionId = project.selections.section
                                    onDispatch(
                                        AppAction.SendRequest(
                                            request {
                                                transport = transportRequest {
                                                    method = Bloop.TransportMethod.QUEUE
                                                    queue = queueRequest {
                                                        this.songId = songId
                                                        this.sectionId = sectionId
                                                    }
                                                }
                                            }
                                        )
                                    )
                                },
                                highlighted = true,
                                iconSize = 28.dp,
                                icon = { modifier ->
                                    Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = "Queue section", modifier = modifier)
                                },
                            )
                        }
                        QueueState.NOT_READY -> {
                            TransportIconButton(
                                onClick = {},
                                enabled = false,
                                iconSize = 28.dp,
                                icon = { modifier ->
                                    Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = "Queue section", modifier = modifier)
                                },
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun MetronomeIndicator(
    isPlaying: Boolean,
    sectionBeat: Double,
) {
    if (!isPlaying) {
        Spacer(modifier = Modifier.height(10.dp))
        return
    }

    val activeBeat = floor(sectionBeat).toInt().mod(4)

    Row(horizontalArrangement = Arrangement.spacedBy(10.dp)) {
        repeat(4) { beatIndex ->
            val isActive = beatIndex == activeBeat
            Box(
                modifier = Modifier
                    .size(width = if (isActive) 22.dp else 14.dp, height = 7.dp)
                    .graphicsLayer {
                        scaleX = if (isActive) 1.12f else 1f
                        scaleY = if (isActive) 1.12f else 1f
                    }
                    .background(
                        color = if (isActive) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.onSurface.copy(alpha = 0.28f),
                        shape = RoundedCornerShape(999.dp),
                    )
            )
        }
    }
}

@Composable
private fun BeatPositionRow(
    isPlaying: Boolean,
    sectionBeat: Double,
    sectionStart: Double,
) {
    if (!isPlaying) {
        Spacer(modifier = Modifier.height(12.dp))
        return
    }

    Row(horizontalArrangement = Arrangement.spacedBy(18.dp)) {
        BeatCounter(label = "Sec", value = floor(sectionBeat).toInt())
        BeatCounter(
            label = "Song",
            value = floor(sectionBeat + sectionStart).toInt(),
            color = MaterialTheme.colorScheme.primary,
        )
    }
}

@Composable
private fun BeatCounter(
    label: String,
    value: Int,
    color: Color = MaterialTheme.colorScheme.onBackground,
) {
    Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            fontWeight = FontWeight.Medium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            text = value.toString(),
            style = MaterialTheme.typography.titleMedium,
            color = color,
            fontFamily = FontFamily.Monospace,
            fontWeight = FontWeight.Bold,
        )
    }
}

@Composable
private fun TransportIconButton(
    onClick: () -> Unit,
    enabled: Boolean = true,
    highlighted: Boolean = false,
    emphasized: Boolean = false,
    iconSize: Dp = 28.dp,
    icon: @Composable (Modifier) -> Unit,
) {
    val contentColor = when {
        emphasized -> MaterialTheme.colorScheme.primary
        highlighted -> MaterialTheme.colorScheme.primary
        else -> MaterialTheme.colorScheme.onSurface
    }
    val disabledColor = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.35f)

    IconButton(
        onClick = onClick,
        enabled = enabled,
        modifier = Modifier.size(if (emphasized) 54.dp else 50.dp),
    ) {
        CompositionLocalProvider(
            androidx.compose.material3.LocalContentColor provides if (enabled) contentColor else disabledColor,
        ) {
            Box(contentAlignment = Alignment.Center) {
                icon(Modifier.size(iconSize))
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
    if (selectedSongIndex == -1) {
        return
    }

    val nextIndex = selectedSongIndex + offset
    if (nextIndex !in project.songsList.indices) {
        return
    }

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

private fun queueState(state: AppState): QueueState {
    val playbackState = state.playbackState
    val selections = state.project.selections
    val invalidId = 0L

    if (playbackState.playing != Bloop.PlayingState.PLAYING) {
        return QueueState.NOT_READY
    }

    val selectedSongId = selections.song
    val selectedSectionId = selections.section

    if (selectedSongId == invalidId || selectedSectionId == invalidId) {
        return QueueState.NOT_READY
    }

    if (
        playbackState.queuedSongId != invalidId &&
        playbackState.queuedSectionId != invalidId &&
        playbackState.queuedSongId == selectedSongId &&
        playbackState.queuedSectionId == selectedSectionId
    ) {
        return QueueState.QUEUED
    }

    if (
        playbackState.songId != invalidId &&
        playbackState.sectionId != invalidId &&
        (playbackState.songId != selectedSongId || playbackState.sectionId != selectedSectionId)
    ) {
        return QueueState.READY
    }

    return QueueState.NOT_READY
}

private enum class QueueState {
    NOT_READY,
    READY,
    QUEUED,
}
