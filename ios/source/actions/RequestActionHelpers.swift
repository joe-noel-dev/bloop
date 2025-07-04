import Foundation

func updateSectionAction(_ section: Bloop_Section) -> Action {
    .sendRequest(
        .with {
            $0.update = .with {
                $0.section = section
            }
        }
    )
}

func updateSongAction(_ song: Bloop_Song) -> Action {
    .sendRequest(
        .with {
            $0.update = .with {
                $0.song = song
            }
        }
    )
}

func updateProjectAction(_ project: Bloop_Project) -> Action {
    .sendRequest(
        .with {
            $0.update = .with {
                $0.project = project
            }
        }
    )
}

func renameProjectAction(projectId: String, name: String) -> Action {
    .sendRequest(
        .with {
            $0.renameProject = .with {
                $0.projectID = projectId
                $0.newName = name
            }
        }
    )
}

func selectSectionAction(_ sectionId: Id) -> Action {
    .sendRequest(
        .with {
            $0.select = .with {
                $0.entity = .section
                $0.id = sectionId
            }
        }
    )
}

func selectSongAction(_ songId: Id) -> Action {
    .sendRequest(
        .with {
            $0.select = .with {
                $0.entity = .song
                $0.id = songId
            }
        }
    )
}

func removeSongAction(_ songId: Id) -> Action {
    .sendRequest(
        .with {
            $0.remove = .with {
                $0.entity = .song
                $0.id = songId
            }
        }
    )
}

func removeSectionAction(_ sectionId: Id) -> Action {
    .sendRequest(
        .with {
            $0.remove = .with {
                $0.entity = .section
                $0.id = sectionId
            }
        }
    )
}

func transportMethodAction(_ method: Bloop_TransportMethod) -> Action {
    .sendRequest(
        .with {
            $0.transport = .with {
                $0.method = method
            }
        }
    )
}

func stopAction() -> Action {
    transportMethodAction(.stop)
}

func playAction() -> Action {
    transportMethodAction(.play)
}

func enterLoopAction() -> Action {
    transportMethodAction(.loop)
}

func exitLoopAction() -> Action {
    transportMethodAction(.exitLoop)
}

func queueAction(song: Id, section: Id) -> Action {
    .sendRequest(
        .with {
            $0.transport = .with {
                $0.method = .queue
                $0.queue = .with {
                    $0.songID = song
                    $0.sectionID = section
                }
            }
        }
    )
}

func addSongAction() -> Action {
    .sendRequest(
        .with {
            $0.add = .with {
                $0.entity = .song
            }
        }
    )
}

func addSectionAction(_ songId: Id) -> Action {
    .sendRequest(
        .with {
            $0.add = .with {
                $0.entity = .section
                $0.id = songId
            }
        }
    )
}

func getProjectsAction() -> Action {
    .sendRequest(
        .with {
            $0.get = .with {
                $0.entity = .projects
            }
        }
    )
}

func removeProjectAction(_ projectId: String) -> Action {
    .sendRequest(
        .with {
            $0.removeProject = .with {
                $0.projectID = projectId
            }
        }
    )
}

func loadProjectAction(_ projectId: String) -> Action {
    .sendRequest(
        .with {
            $0.load = .with {
                $0.projectID = projectId
            }
        }
    )
}

func newProjectAction() -> Action {
    .sendRequest(
        .with {
            $0.add = .with {
                $0.entity = .project
            }
        }
    )
}

func duplicateProjectAction(_ projectId: String) -> Action {
    .sendRequest(
        .with {
            $0.duplicateProject = .with {
                $0.projectID = projectId
            }
        }
    )
}

func pushProjectAction(_ projectId: String) -> Action {
    .sendRequest(
        .with {
            $0.projectSync = .with {
                $0.projectID = projectId
                $0.method = .push
            }
        }
    )
}

func pullProjectAction(_ projectId: String) -> Action {
    .sendRequest(
        .with {
            $0.projectSync = .with {
                $0.projectID = projectId
                $0.method = .pull
            }
        }
    )
}

func getWaveformAction(_ sampleId: Id) -> Action {
    .sendRequest(
        .with {
            $0.get = .with {
                $0.entity = .waveform
                $0.id = sampleId
            }
        }
    )
}

func logInAction(email: String, password: String) -> Action {
    .sendRequest(
        .with {
            $0.login = .with {
                $0.username = email
                $0.password = password
            }
        }
    )
}

func logOutAction() -> Action {
    .sendRequest(
        .with {
            $0.logout = .init()
        }
    )
}

