## Experiment 3.1: Original Code

![3.1 1](assets/images/3_1_1.png)
![3.1 2](assets/images/3_1_2.png)
![3.1 3](assets/images/3_1_3.png)
![3.1 4](assets/images/3_1_4.png)


## Experiment 3.2: Be Creative!

![3.2 1](assets/images/3_2_1.png)
![3.2 2](assets/images/3_2_2.png)
![3.2 3](assets/images/3_2_3.png)

### Modifikasi yang Dilakukan

**1. Redesign UI**:
Mengubah tampilan dengan style dan color palette baru: oranye (#FF6B35), kuning (#FFD93D), 
biru muda (#4DC9E6), border hitam tebal, dan box shadow. Perubahan diterapkan pada halaman login, header, sidebar, message bubble, dan input area.

**2. Avatar**:
Update URL DiceBear lama  ke API baru:
`https://api.dicebear.com/9.x/adventurer-neutral/svg?seed={username}`

**3. Posisi Bubble Pesan**:
Bubble pesan milik sendiri tampil di kanan, sedangkan bubble pesan orang lain di kiri. Implementasi dengan mengecek `m.from == self.username`.

**4. Auto-render Gambar**:
Link berakhiran `.gif`, `.jpg`, `.jpeg`, `.png` otomatis ditampilkan sebagai gambar/GIF di bubble chat.

**5. Logged In As**:
Header menampilkan username yang sedang login di pojok kanan atas.