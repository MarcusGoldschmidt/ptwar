package pkg

import (
	"runtime"
	"runtime/metrics"
)

type ServerStats struct {
	MemStats runtime.MemStats      `json:"memStats"`
	Metrics  []metrics.Description `json:"metrics"`
}

type ServerVersionInfo struct {
	Version string `json:"version"`
	Commit  string `json:"commit"`
	BuiltBy string `json:"builtBy"`
	BuiltAt string `json:"builtAt"`
}

// Memory returns the memory stats of the server
func (s *PtwarGameServer) Memory() ServerStats {
	var stats runtime.MemStats
	runtime.ReadMemStats(&stats)

	return ServerStats{
		MemStats: stats,
		Metrics:  metrics.All(),
	}
}

// Version returns the version info of the server
func (s *PtwarGameServer) Version() ServerVersionInfo {
	return ServerVersionInfo{
		Version: Version,
		Commit:  Commit,
		BuiltBy: BuiltBy,
		BuiltAt: BuiltAt,
	}
}
