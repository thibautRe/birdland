import Phaser from 'phaser'

import Bird from './bird'
import Plant from './scenery/plants'
import Seeds from './collectables/seeds'
import FloorTile from './scenery/floortile'
import Grid from './scenery/grid'

export default class Scene extends Phaser.Scene {
  constructor() {
    super({
      key: 'MainScene',
    })
  }

  preload() {
    Bird.preload(this)
    Plant.preload(this)
    Seeds.preload(this)
  }

  create() {
    // Create floor tiles
    new Grid({ scene: this, height: 100, width: 100 })

    // Create plants
    const plants = [{ x: 100, y: 200 }, { x: 164, y: 200 }]
    plants.forEach(plant => new Plant({ scene: this, ...plant }))

    new Seeds({
      scene: this,
      x: 100,
      y: 300,
    })

    this.bird = new Bird({
      scene: this,
      x: 100,
      y: 100,
    })

    this.cameras.main.setBackgroundColor('#a2dbf9')

    this.cursors = this.input.keyboard.createCursorKeys()
  }

  update() {
    const { cursors } = this

    this.bird.update({
      cursors,
    })
  }
}
