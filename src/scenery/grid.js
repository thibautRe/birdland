import Phaser from 'phaser'
import FloorTile from './floortile'

export default class Grid extends Phaser.GameObjects.Group {
  constructor({ scene, height, width }) {
    super(scene)

    this.tileHeight = 50
    this.tileWidth = 50

    for (let i = 0; i < width - 1; i++) {
      for (let j = 0; j < height - 1; j++) {
        this.add(
          new FloorTile({
            x: this.tileWidth * i,
            y: this.tileHeight * j,
            width: this.tileWidth,
            height: this.tileHeight,
            scene,
          }),
        )
      }
    }
  }
}
