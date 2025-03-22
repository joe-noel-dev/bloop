import Foundation

func updateSectionAction(_ section: Bloop_Section) -> Action {
    .sendRequest(.with {
        $0.update = .with {
            $0.section = section
        }
    })
}

func updateSongAction(_ song: Bloop_Song) -> Action {
    .sendRequest(.with {
        $0.update = .with {
            $0.song = song
        }
    })
}

func updateProjectAction(_ project: Bloop_Project) -> Action {
    .sendRequest(.with {
        $0.update = .with {
            $0.project = project
        }
    })
}

func renameProjectAction(_ name: String) -> Action {
    .sendRequest(.with {
        $0.rename = .with {
            $0.entity = .project
            $0.name = name
        }
    })
}

func selectSectionAction(_ sectionId: Id) -> Action {
    .sendRequest(.with {
        $0.select = .with {
            $0.entity = .section
            $0.id = sectionId
        }
    })
}

func selectSongAction(_ songId: Id) -> Action {
    .sendRequest(.with {
        $0.select = .with {
            $0.entity = .song
            $0.id = songId
        }
    })
}

func removeSongAction(_ songId: Id) -> Action {
    .sendRequest(.with {
        $0.remove = .with {
            $0.entity = .song
            $0.id = songId
        }
    })
}

func removeSectionAction(_ sectionId: Id) -> Action {
    .sendRequest(.with {
        $0.remove = .with {
            $0.entity = .section
            $0.id = sectionId
        }
    })
}

func transportMethodAction(_ method: Bloop_TransportMethod) -> Action {
    .sendRequest(.with {
        $0.transport = .with {
            $0.method = method
        }
    })
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
    .sendRequest(.with {
        $0.transport = .with {
            $0.method = .queue
            $0.queue = .with {
                $0.songID = song
                $0.sectionID = section
            }
        }
    })
}

func addSongAction() -> Action {
    .sendRequest(.with {
        $0.add = .with {
            $0.entity = .song
        }
    })
}

func addSectionAction(_ songId: Id) -> Action {
    .sendRequest(.with {
        $0.add = .with {
            $0.entity = .section
            $0.id = songId
        }
    })
}

func getProjectsAction() -> Action {
    .sendRequest(.with {
        $0.get = .with {
            $0.entity = .projects
        }
    })
}

func removeProjectAction(_ projectId: Id) -> Action {
    .sendRequest(.with {
        $0.remove = .with {
            $0.entity = .project
            $0.id = projectId
        }
    })
}

func loadProjectAction(_ projectId: Id) -> Action {
    .sendRequest(.with {
        $0.load = .with {
            $0.id = projectId
        }
    })
}

func newProjectAction() -> Action {
    .sendRequest(.with {
        $0.add = .with {
            $0.entity = .project
        }
    })
}

func duplicateProjectAction(_ projectId: Id) -> Action {
    .sendRequest(.with {
        $0.duplicate = .with {
            $0.entity = .project
            $0.id = projectId
        }
    })
}

func getWaveformAction(_ sampleId: Id) -> Action {
    .sendRequest(.with {
        $0.get = .with {
            $0.entity = .waveform
            $0.id = sampleId
        }
    })
}
