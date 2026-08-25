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
import androidx.compose.material.icons.filled.Cloud
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Error
import androidx.compose.material.icons.filled.Smartphone
import androidx.compose.material.icons.filled.Sync
import androidx.compose.material3.AlertDialog
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
import androidx.compose.material3.Text
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
import bloop.removeProjectRequest
import bloop.request
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState
import java.time.Instant
import java.time.format.DateTimeParseException

private data class AvailableProject(
    val project: Bloop.ProjectInfo,
    val isLocal: Boolean,
    val isCloud: Boolean,
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun ProjectsSheet(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    onDismiss: () -> Unit,
) {
    var selectedProjectId by remember { mutableStateOf<String?>(null) }
    var projectToDelete by remember { mutableStateOf<AvailableProject?>(null) }

    val availableProjects = remember(state.projects, state.cloudProjects) {
        val localProjects = state.projects.associateBy { it.id }
        val cloudProjects = state.cloudProjects.associateBy { it.id }
        (localProjects.keys + cloudProjects.keys).map { id ->
            AvailableProject(
                project = cloudProjects[id] ?: localProjects.getValue(id),
                isLocal = localProjects.containsKey(id),
                isCloud = cloudProjects.containsKey(id),
            )
        }.sortedByDescending { it.project.lastSaved }
    }
    val allProjects = remember(state.projects, state.cloudProjects) {
        state.projects + state.cloudProjects
    }

    LaunchedEffect(state.projects, state.cloudProjects) {
        selectedProjectId?.let { selectedId ->
            if (availableProjects.none { it.project.id == selectedId }) selectedProjectId = null
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
                if (availableProjects.isNotEmpty()) {
                    item {
                        Text(
                            text = "Projects",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                        )
                    }
                    items(availableProjects, key = { it.project.id }) { availableProject ->
                        val isSelected = selectedProjectId == availableProject.project.id
                        ProjectItem(
                            project = availableProject.project,
                            isSelected = isSelected,
                            isLocal = availableProject.isLocal,
                            isCloud = availableProject.isCloud,
                            onClick = {
                                selectedProjectId = if (isSelected) null else availableProject.project.id
                            },
                        )
                    }
                }

                if (availableProjects.isEmpty()) {
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

            selectedProjectId?.let { selectedId ->
                val availableProject = availableProjects.first { it.project.id == selectedId }
                HorizontalDivider()
                ProjectActionBar(
                    onOpen = {
                        onDispatch(
                            AppAction.SendRequest(
                                request { load = loadProjectRequest { projectId = selectedId } }
                            )
                        )
                        onDismiss()
                    },
                    onDuplicate = if (availableProject.isLocal) {
                        {
                            onDispatch(
                                AppAction.SendRequest(
                                    request {
                                        duplicateProject = duplicateProjectRequest { projectId = selectedId }
                                    }
                                )
                            )
                            onDismiss()
                        }
                    } else null,
                    onDelete = { projectToDelete = availableProject },
                )
            }
        }
    }

    projectToDelete?.let { availableProject ->
        AlertDialog(
            onDismissRequest = { projectToDelete = null },
            title = { Text("Delete ${availableProject.project.name}?") },
            text = {
                Text(
                    if (availableProject.isLocal && availableProject.isCloud) {
                        "Choose whether to remove only the offline download or delete the project everywhere."
                    } else if (availableProject.isCloud) {
                        "This deletes the project from the cloud."
                    } else {
                        "This removes the project from this device."
                    }
                )
            },
            confirmButton = {
                if (availableProject.isCloud) {
                    Button(onClick = {
                        onDispatch(removeProjectRequestAction(availableProject.project.id, removeCloud = true))
                        projectToDelete = null
                    }) { Text("Delete Project") }
                }
            },
            dismissButton = {
                if (availableProject.isLocal) {
                    OutlinedButton(onClick = {
                        onDispatch(removeProjectRequestAction(availableProject.project.id, removeCloud = false))
                        projectToDelete = null
                    }) { Text("Remove Download") }
                }
            },
        )
    }
}

private fun removeProjectRequestAction(projectId: String, removeCloud: Boolean) = AppAction.SendRequest(
    request {
        removeProject = removeProjectRequest {
            this.projectId = projectId
            targets += Bloop.ProjectRemovalTarget.PROJECT_REMOVAL_TARGET_LOCAL
            if (removeCloud) targets += Bloop.ProjectRemovalTarget.PROJECT_REMOVAL_TARGET_REMOTE
        }
    }
)

@Composable
private fun ProjectActionBar(
    onOpen: () -> Unit,
    onDuplicate: (() -> Unit)?,
    onDelete: () -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 12.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Button(onClick = onOpen, modifier = Modifier.weight(1f)) {
            Text("Open")
        }
        if (onDuplicate != null) {
            OutlinedButton(onClick = onDuplicate, modifier = Modifier.weight(1f)) {
                Text("Duplicate")
            }
        }
        OutlinedButton(onClick = onDelete, modifier = Modifier.weight(1f)) {
            Icon(Icons.Filled.Delete, contentDescription = "Delete")
        }
    }
}

@Composable
private fun ProjectItem(
    project: Bloop.ProjectInfo,
    isSelected: Boolean,
    isLocal: Boolean,
    isCloud: Boolean,
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
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp), verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        text = project.name.takeIf { it.isNotBlank() } ?: "Untitled",
                        fontWeight = if (isSelected) FontWeight.SemiBold else FontWeight.Normal,
                    )
                    if (isLocal) Icon(Icons.Filled.Smartphone, contentDescription = "Available offline")
                    if (isCloud) Icon(Icons.Filled.Cloud, contentDescription = "Available in cloud")
                }
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
