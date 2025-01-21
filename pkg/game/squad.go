package game

type InstructionState int

const (
	InstructionStateNormal InstructionState = iota
	InstructionStateMoving
	InstructionStateAttacking
	InstructionStateBuilding
	InstructionStateRepairing
	InstructionStateUpgrading
	InstructionStateTraining
	InstructionStateHealing
	InstructionStateGarrisoning
	InstructionStateRetreating
	InstructionStatePatrolling
	InstructionStateScouting
	InstructionStateSieging
	InstructionStateCapturing
	InstructionStateDefending
	InstructionStatePartisan
)

type Helmet struct{}

type Armor struct{}

type Weapon struct{}

type SpecialKit struct{}

type Soldier struct {
	Id      uint64
	SquadId uint64
	Name    string
	Age     int8
	MaxLife float64
	Life    float64
	Level   int8
	Xp      float64

	// Soldier Layout
	Helmet     *Helmet
	Armor      *Armor
	Weapon     *Weapon
	SpecialKit *SpecialKit
}

type Squad struct {
	Id               uint64
	PlayerId         uint64
	Name             string
	SoldiersById     map[uint64]*Soldier
	InstructionState InstructionState

	// Attack Squad stats
	Speed      float64
	SoftAttack float32
	HardAttack float32
	Defense    float32
	Armor      float32
	Piercing   float32

	// Building Squad stats
	BuildSpeed float64
}

func (s *Squad) Width() int {
	return len(s.SoldiersById)
}

func (s *Squad) LifeRatio() float64 {
	return s.Life() / s.MaxLife()
}

func (s *Squad) MaxLife() float64 {
	var maxLife float64
	for _, soldier := range s.SoldiersById {
		maxLife += soldier.MaxLife
	}

	return maxLife
}

func (s *Squad) Life() float64 {
	var life float64
	for _, soldier := range s.SoldiersById {
		life += soldier.Life
	}

	return life
}
