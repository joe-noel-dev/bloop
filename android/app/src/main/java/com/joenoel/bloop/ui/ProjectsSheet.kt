package com.joenoel.bloop.ui

import androidx.compose.animation.core.LinearEasing
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.Help
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Error
import androidx.compose.material.icons.filled.Sync
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Surface
import androidx.compose.material3.SwipeToDismissBox
import androidx.compose.material3.SwipeToDismissBoxValue
import androidx.compose.material3.Text
import androidx.compose.material3.rememberSwipeToDismissBoxState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import bloop.Bloop
import bloop.addRequest
import bloop.duplicateProjectRequest
import bloop.getRequest
import bloop.loadProjectRequest
import bloop.projectSyncRequest
import bloop.removeProjectRequest
import bloop.request
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState
import java.time.Instant
import java.time.format.DateTimeParseException

private sealed interface ProjectLocation {
    val id: String

    data class Local(override val id: String) : ProjectLocation
    data class Cloud(override val id: String) : ProjectLocation
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun ProjectsSheet(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    onDismiss: () -> Unit,
) {
    var selected by remember { mutableStateOf<ProjectLocation?>(null) }

    val sortedProjects = remember(state.projects) {
        state.projects.sortedByDescending { it.lastSaved }
    }
    val sortedCloudProjects = remember(state.cloudProjects) {
        state.cloudProjects.sortedByDescending { it.lastSaved }
    }
    val allProjects = remember(state.projects, state.cloudProjects) {
        state.projects + state.cloudProjects
    }

    LaunchedEffect(state.projects, state.cloudProjects) {
        selected?.let { sel ->
            val exists = when (sel) {
                is ProjectLocation.Local -> state.projects.any { it.id == sel.id }
                is ProjectLocation.Cloud -> state.cloudProjects.any { it.id == sel.id }
            }
            if (!exists) selected = null
        }
    }
    LaunchedEffect(Unit) {
        onDispatch(
            AppAction.SendRequest(
                request { get = getRequest { entity = Bloop.Entity.PROJECTS } }
            )
        )
    }

    ModalBottomSheet(onDismissRequest = onDismiss) {
        Column(modifier = Modifier.navigationBarsPadding()) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp, vertical = 12.dp),
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = "Projects",
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                    modifier = Modifier.weight(1f),
                )
                IconButton(
                    onClick = {
                        onDispatch(
                            AppAction.SendRequest(
                                request { add = addRequest { entity = Bloop.Entity.PROJECT } }
                            )
                        )
                        onDismiss()
                    },
                ) {
                    Icon(Icons.Filled.Add, contentDescription = "New project")
                }
            }

            state.projectSyncStatuses.forEach { (projectId, syncStatus) ->
                ProjectSyncNotificationItem(
                    projectId = projectId,
                    syncStatus = syncStatus,
                    projects = allProjects,
                    onDismiss = { onDispatch(AppAction.DismissProjectSync(projectId)) },
                    modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp),
                )
            }

            if (state.projectSyncStatuses.isNotEmpty()) {
                HorizontalDivider()
            }

            LazyColumn {
                if (sortedProjects.isNotEmpty()) {
                    item {
                        Text(
                            text = "Local",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                        )
                    }
                    items(sortedProjects, key = { it.id }) { project ->
                        val isSelected = selected?.id == project.id && selected is ProjectLocation.Local
                        val dismissState = rememberSwipeToDismissBoxState(
                            confirmValueChange = { value ->
                                if (value == SwipeToDismissBoxValue.EndToStart) {
                                    onDispatch(
                                        AppAction.SendRequest(
                                            request {
                                                removeProject = removeProjectRequest { projectId = project.id }
                                            }
                                        )
                                    )
                                    true
                                } else {
                                    false
                                }
                            },
                            positionalThreshold = { it * 0.4f },
                        )
                        SwipeToDismissBox(
                            state = dismissState,
                            backgroundContent = {
                                Box(
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .padding(horizontal = 16.dp),
                                    contentAlignment = Alignment.CenterEnd,
                                ) {
                                    Icon(
                                        Icons.Filled.Close,
                                        contentDescription = "Delete",
                                        tint = MaterialTheme.colorScheme.error,
                                    )
                                }
                            },
                            enableDismissFromStartToEnd = false,
                        ) {
                            ProjectItem(
                                project = project,
                                isSelected = isSelected,
                                onClick = {
                                    selected = if (isSelected) null else ProjectLocation.Local(project.id)
                                },
                            )
                        }
                    }
                }

                if (sortedCloudProjects.isNotEmpty()) {
                    item {
                        Text(
                            text = "Cloud",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(start = 16.dp, top = 12.dp, end = 16.dp, bottom = 8.dp),
                        )
                    }
                    items(sortedCloudProjects, key = { "cloud_${it.id}" }) { project ->
                        val isSelected = selected?.id == project.id && selected is ProjectLocation.Cloud
                        ProjectItem(
                            project = project,
                            isSelected = isSelected,
                            onClick = {
                                selected = if (isSelected) null else ProjectLocation.Cloud(project.id)
                            },
                        )
                    }
                }

                if (sortedProjects.isEmpty() && sortedCloudProjects.isEmpty()) {
                    item {
                        Text(
                            text = "No projects yet. Tap + to create one.",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(16.dp),
                        )
                    }
                }
            }

            selected?.let { location ->
                HorizontalDivider()
                ProjectActionBar(
                    location = location,
                    onOpen = {
                        onDispatch(
                            AppAction.SendRequest(
                                request { load = loadProjectRequest { projectId = location.id } }
                            )
                        )
                        onDismiss()
                    },
                    onDuplicate = {
                        onDispatch(
                            AppAction.SendRequest(
                                request {
                                    duplicateProject = duplicateProjectRequest { projectId = location.id }
                                }
                            )
                        )
                        onDismiss()
                    },
                    onPush = {
                        onDispatch(
                            AppAction.SendRequest(
                                request {
                                    projectSync = projectSyncRequest {
                                        projectId = location.id
                                        method = Bloop.SyncMethod.SYNC_METHOD_PUSH
                                    }
                                }
                            )
                        )
                    },
                    onPull = {
                        onDispatch(
                            AppAction.SendRequest(
                                request {
                                    projectSync = projectSyncRequest {
                                        projectId = location.id
                                        method = Bloop.SyncMethod.SYNC_METHOD_PULL
                                    }
                                }
                            )
                        )
                    },
                )
            }
        }
    }
}

@Composable
private fun ProjectActionBar(
    location: ProjectLocation,
    onOpen: () -> Unit,
    onDuplicate: () -> Unit,
    onPush: () -> Unit,
    onPull: () -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 12.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        when (location) {
            is ProjectLocation.Local -> {
                Button(onClick = onOpen, modifier = Modifier.weight(1f)) {
                    Text("Open")
                }
                OutlinedButton(onClick = onDuplicate, modifier = Modifier.weight(1f)) {
                    Text("Duplicate")
                }
                OutlinedButton(onClick = onPush, modifier = Modifier.weight(1f)) {
                    Text("Push")
                }
            }
            is ProjectLocation.Cloud -> {
                Button(onClick = onPull, modifier = Modifier.weight(1f)) {
                    Text("Pull")
                }
            }
        }
    }
}

@Composable
private fun ProjectItem(
    project: Bloop.ProjectInfo,
    isSelected: Boolean,
    onClick: () -> Unit,
) {
    Surface(
        color = if (isSelected) {
            MaterialTheme.colorScheme.primaryContainer
        } else {
            MaterialTheme.colorScheme.surface
        },
    ) {
        ListItem(
            headlineContent = {
                Text(
                    text = project.name.takeIf { it.isNotBlank() } ?: "Untitled",
                    fontWeight = if (isSelected) FontWeight.SemiBold else FontWeight.Normal,
                )
            },
            supportingContent = if (isSelected && project.lastSaved.isNotBlank()) {
                {
                    Text(
                        text = "Last saved ${formatRelativeTime(project.lastSaved)}",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            } else {
                null
            },
            modifier = Modifier.clickable(onClick = onClick),
        )
    }
}

@Composable
private fun ProjectSyncNotificationItem(
    projectId: String,
    syncStatus: Bloop.SyncStatus,
    projects: List<Bloop.ProjectInfo>,
    onDismiss: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val projectName = projects.firstOrNull { it.id == projectId }?.name ?: "Unknown Project"

    val statusColor = when (syncStatus) {
        Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS -> MaterialTheme.colorScheme.primary
        Bloop.SyncStatus.SYNC_STATUS_COMPLETE -> MaterialTheme.colorScheme.tertiary
        Bloop.SyncStatus.SYNC_STATUS_ERROR -> MaterialTheme.colorScheme.error
        else -> MaterialTheme.colorScheme.outline
    }

    val statusText = when (syncStatus) {
        Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS -> "Syncing..."
        Bloop.SyncStatus.SYNC_STATUS_COMPLETE -> "Sync completed"
        Bloop.SyncStatus.SYNC_STATUS_ERROR -> "Sync failed"
        else -> "Unknown status"
    }

    val rotationAngle = if (syncStatus == Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS) {
        val infiniteTransition = rememberInfiniteTransition(label = "sync_spin_$projectId")
        val angle by infiniteTransition.animateFloat(
            initialValue = 0f,
            targetValue = 360f,
            animationSpec = infiniteRepeatable(
                animation = tween(durationMillis = 2000, easing = LinearEasing),
                repeatMode = RepeatMode.Restart,
            ),
            label = "rotation",
        )
        angle
    } else {
        0f
    }

    Surface(
        color = statusColor.copy(alpha = 0.1f),
        shape = MaterialTheme.shapes.small,
        modifier = modifier.fillMaxWidth(),
    ) {
        Row(
            modifier = Modifier.padding(12.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            val iconModifier = if (syncStatus == Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS) {
                Modifier.graphicsLayer { rotationZ = rotationAngle }
            } else {
                Modifier
            }
            Icon(
                imageVector = when (syncStatus) {
                    Bloop.SyncStatus.SYNC_STATUS_IN_PROGRESS -> Icons.Filled.Sync
                    Bloop.SyncStatus.SYNC_STATUS_COMPLETE -> Icons.Filled.CheckCircle
                    Bloop.SyncStatus.SYNC_STATUS_ERROR -> Icons.Filled.Error
                    else -> Icons.AutoMirrored.Filled.Help
                },
                contentDescription = statusText,
                tint = statusColor,
                modifier = iconModifier,
            )
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = projectName,
                    style = MaterialTheme.typography.bodyMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    text = statusText,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            IconButton(onClick = onDismiss) {
                Icon(
                    Icons.Filled.Close,
                    contentDescription = "Dismiss",
                    tint = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

private fun formatRelativeTime(rfc3339: String): String {
    return try {
        val instant = Instant.parse(rfc3339)
        val now = Instant.now()
        val secondsDiff = now.epochSecond - instant.epochSecond
        if (secondsDiff < 0) {
            val secondsUntil = -secondsDiff
            when {
                secondsUntil < 60 -> "in <1m"
                secondsUntil < 3600 -> "in ${secondsUntil / 60}m"
                secondsUntil < 86400 -> "in ${secondsUntil / 3600}h"
                secondsUntil < 604800 -> "in ${secondsUntil / 86400}d"
                else -> "in ${secondsUntil / 604800}w"
            }
        } else {
            when {
                secondsDiff < 60 -> "just now"
                secondsDiff < 3600 -> "${secondsDiff / 60}m ago"
                secondsDiff < 86400 -> "${secondsDiff / 3600}h ago"
                secondsDiff < 604800 -> "${secondsDiff / 86400}d ago"
                else -> "${secondsDiff / 604800}w ago"
            }
        }
    } catch (_: DateTimeParseException) {
        rfc3339
    }
}
