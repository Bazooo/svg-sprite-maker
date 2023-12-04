<script lang="ts">
    import { applicationSettings } from '../store'

    $: g1 = $applicationSettings?.transparencyGridColor1 ?? ['black', 0.1]
    $: g2 = $applicationSettings?.transparencyGridColor2 ?? ['transparent', 0]
</script>

<svg class="absolute left-0 top-0 z-0 h-full w-full" preserveAspectRatio="xMidYMid slice">
    <defs>
        <pattern id="sprite-maker-transparent-grid" width="20" height="20" patternUnits="userSpaceOnUse">
            <rect fill={g1[0]} x="0" y="0" width="10" height="10" opacity={g1[1]} />
            <rect fill={g1[0]} x="10" y="10" width="10" height="10" opacity={g1[1]} />
            <rect fill={g2[0]} x="10" y="0" width="10" height="10" opacity={g2[1]} />
            <rect fill={g2[0]} x="0" y="10" width="10" height="10" opacity={g2[1]} />
        </pattern>
    </defs>
    <rect fill="url(#sprite-maker-transparent-grid)" x="0" y="0" width="100%" height="100%" />
</svg>
