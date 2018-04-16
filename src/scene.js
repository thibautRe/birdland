import Phaser from 'phaser'

import Bird from './bird'
import Plant from './scenery/plants'

export default class Scene extends Phaser.Scene {
  constructor() {
    super({
      key: 'MainScene',
    })
  }

  preload() {
    Bird.preload(this)
    Plant.preload(this)
  }

  create() {
    // Create Scenery
    const plants = [{ x: 100, y: 200 }, { x: 164, y: 200 }]
    plants.forEach(plant => new Plant({ scene: this, ...plant }))

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
