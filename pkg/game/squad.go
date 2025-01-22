package game

import (
	"context"
)

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

type ModifierType string

const (
	// ModifierTypePercentage 0 to 100
	ModifierTypePercentage  ModifierType = "Percentage"
	ModifierTypeMultiplier  ModifierType = "Multiplier"
	ModifierTypeAddition    ModifierType = "Addition"
	ModifierTypeSubtraction ModifierType = "Subtraction"
	ModifierTypeDivision    ModifierType = "Division"
)

type UnityStats struct {
	Speed      float32
	SoftAttack float32
	HardAttack float32
	Defense    float32
	Armor      float32
	Piercing   float32
	Weight     float32

	// Building float32 stats
	BuildSpeed float32
}
type SoldierModifier struct {
	Id          uint64
	Name        string
	Description string

	Value float32
	Type  ModifierType
	Apply func(s *Soldier)
}

func (sm *SoldierModifier) ApplyValue(base float32) float32 {
	switch sm.Type {
	case ModifierTypePercentage:
		if sm.Value > 100 || sm.Value < 0 {
			return base
		}
		return base * (1 + (sm.Value / 100))
	case ModifierTypeMultiplier:
		return base * sm.Value
	case ModifierTypeAddition:
		return base + sm.Value
	case ModifierTypeSubtraction:
		return base - sm.Value
	case ModifierTypeDivision:
		return base / sm.Value
	}

	return base
}

type Helmet struct {
	Id      uint64
	Name    string
	Defense float32
	Armor   float32
	Weight  float32
}

type Armor struct {
	Id      uint64
	Name    string
	Defense float32
	Armor   float32
	Weight  float32
}

type Weapon struct {
	Id         uint64
	Name       string
	SoftAttack float32
	HardAttack float32
	Piercing   float32
	Weight     float32
}

type SpecialKit struct {
	Id          uint64
	Name        string
	Description string
	Modifiers   []*SoldierModifier
	Weight      float32
}

type Soldier struct {
	Id      uint64
	SquadId uint64
	Name    string
	Age     int8
	MaxLife float32
	Life    float32
	Level   int8
	Xp      float32

	BaseAccuracy float32
	BaseSpeed    float32

	// Soldier Layout
	Helmet      *Helmet
	Armor       *Armor
	Weapon      *Weapon
	SpecialKit1 *SpecialKit
	SpecialKit2 *SpecialKit

	// Stats
	Stats UnityStats
}

func (s *Soldier) ApplyModifiers() {
	s.Stats = UnityStats{
		Speed:      1,
		SoftAttack: 1,
		HardAttack: 0,
		Defense:    1,
		Armor:      0,
		Piercing:   0,
		BuildSpeed: 0.5,
		Weight:     1,
	}

	if s.Helmet != nil {
		s.Stats.Defense += s.Helmet.Defense
		s.Stats.Armor += s.Helmet.Armor
		s.Stats.Weight += s.Helmet.Weight
	}

	if s.Armor != nil {
		s.Stats.Defense += s.Armor.Defense
		s.Stats.Armor += s.Armor.Armor
		s.Stats.Weight += s.Armor.Weight
	}

	if s.Weapon != nil {
		s.Stats.SoftAttack += s.Weapon.SoftAttack
		s.Stats.HardAttack += s.Weapon.HardAttack
		s.Stats.Piercing += s.Weapon.Piercing
		s.Stats.Weight += s.Weapon.Weight
	}

	if s.SpecialKit1 != nil {
		for _, modifier := range s.SpecialKit1.Modifiers {
			modifier.Apply(s)
		}
	}

	if s.SpecialKit2 != nil {
		for _, modifier := range s.SpecialKit2.Modifiers {
			modifier.Apply(s)
		}
	}
}

type Squad struct {
	Id               uint64
	PlayerId         uint64
	Name             string
	SoldiersById     map[uint64]*Soldier
	InstructionState InstructionState

	// Attack Squad stats
	Stats UnityStats

	// Cache
	maxLifeCache float32
	lifeCache    float32
}

func (s *Squad) CalculateCache(ctx context.Context) {
	{
		var maxLife float32
		for _, soldier := range s.SoldiersById {
			maxLife += soldier.MaxLife
		}
		s.maxLifeCache = maxLife
	}

	{
		var life float32
		for _, soldier := range s.SoldiersById {
			life += soldier.Life
		}

		s.lifeCache = life
	}
}

func (s *Squad) Width() int {
	return len(s.SoldiersById)
}

func (s *Squad) LifeRatio() float32 {
	return s.Life() / s.MaxLife()
}

func (s *Squad) MaxLife() float32 {
	return s.maxLifeCache
}

func (s *Squad) Life() float32 {
	return s.lifeCache
}
