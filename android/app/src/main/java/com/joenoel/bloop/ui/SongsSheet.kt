package com.joenoel.bloop.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowRight
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import bloop.Bloop
import bloop.request
import bloop.selectRequest
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun SongsSheet(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    onDismiss: () -> Unit,
) {
    ModalBottomSheet(onDismissRequest = onDismiss) {
        Column(modifier = Modifier.navigationBarsPadding()) {
            Text(
                text = "Songs",
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.SemiBold,
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 12.dp),
            )
            HorizontalDivider()
            LazyColumn {
                items(state.project.songsList, key = { it.id }) { song ->
                    val isSelected = song.id == state.project.selections.song
                    ListItem(
                        headlineContent = {
                            Text(
                                text = song.name.takeIf { it.isNotBlank() } ?: "Untitled",
                                color = if (isSelected) {
                                    MaterialTheme.colorScheme.primary
                                } else {
                                    MaterialTheme.colorScheme.onSurface
                                },
                            )
                        },
                        trailingContent = {
                            if (isSelected) {
                                Icon(
                                    imageVector = Icons.Filled.CheckCircle,
                                    contentDescription = "Selected",
                                    tint = MaterialTheme.colorScheme.primary,
                                )
                            } else {
                                Icon(
                                    imageVector = Icons.AutoMirrored.Filled.KeyboardArrowRight,
                                    contentDescription = null,
                                    tint = MaterialTheme.colorScheme.onSurface.copy(alpha = 0.4f),
                                )
                            }
                        },
                        modifier = Modifier.clickable {
                            onDispatch(
                                AppAction.SendRequest(
                                    request {
                                        select = selectRequest {
                                            entity = Bloop.Entity.SONG
                                            id = song.id
                                        }
                                    }
                                )
                            )
                            onDismiss()
                        },
                    )
                }
            }
        }
    }
}
